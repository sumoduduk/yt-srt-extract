use std::{
    fs::File,
    io::Write,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use youtube_transcript::YoutubeBuilder;

pub async fn extract_srt(
    url: &str,
    format_res: &str,
    path_name: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let yt = YoutubeBuilder::default();
    let builder = yt.build();

    let srt = builder.transcript(url).await?;

    let file_name_str = get_seconds();

    match format_res {
        "json" => {
            let json = serde_json::to_string_pretty(&srt)?;

            let file_name = format!("{}.json", file_name_str);

            let file_path = path_name.join(&file_name);

            let mut file = File::create(file_path)?;

            println!("transcript : {}", &json);

            file.write_all(json.as_bytes())?;
        }
        "text" => {
            let caption = srt
                .transcripts
                .into_iter()
                .map(|t| t.text)
                .collect::<Vec<String>>();

            let res = caption.join(" ");
            println!("{}", &res);

            let file_name = format!("{}.txt", file_name_str);

            let file_path = path_name.join(&file_name);

            let mut file = File::create(file_path)?;
            file.write_all(res.as_bytes())?;
        }
        _ => return Err("Need json or text".into()),
    }

    Ok(())
}

fn get_seconds() -> u64 {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seconds = current_time.as_secs();
    seconds
}
