use std::{
    fs::File,
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

use youtube_transcript::YoutubeBuilder;

pub async fn extract_srt(url: &str, format_res: &str) -> Result<(), Box<dyn std::error::Error>> {
    let yt = YoutubeBuilder::default();
    let builder = yt.build();

    let srt = builder.transcript(url).await?;

    let file_name_str = get_seconds();

    match format_res {
        "json" => {
            let json = serde_json::to_string_pretty(&srt)?;

            let file_name = format!("results/{}.json", file_name_str);

            let mut file = File::create(file_name)?;

            println!("transcript : {}", &json);

            file.write_all(json.as_bytes())?;
        }
        "text" => {
            let caption = srt
                .transcripts
                .into_iter()
                .map(|t| t.text)
                .collect::<Vec<String>>();

            let caption = caption.join(" ");
            println!("{}", &caption);

            let file_name = format!("results/{}.txt", file_name_str);

            let mut file = File::create(&file_name)?;
            file.write_all(caption.as_bytes())?;
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
