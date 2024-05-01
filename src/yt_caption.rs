use youtube_transcript::YoutubeBuilder;

pub async fn extract_srt(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let yt = YoutubeBuilder::default();
    let builder = yt.build();

    let srt = builder.transcript(url).await?;

    let caption = srt
        .transcripts
        .into_iter()
        .map(|t| t.text)
        .collect::<Vec<String>>();

    let caption = caption.join(" ");
    println!("{caption}");

    // let json = serde_json::to_string_pretty(&srt)?;
    // println!("transcript : {}", &json);
    Ok(())
}
