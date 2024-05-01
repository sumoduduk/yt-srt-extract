mod yt_caption;
use std::env::args;
use yt_caption::extract_srt;

#[tokio::main]
async fn main() {
    let mut args = args();

    args.next();

    let link = args.next().unwrap();
    extract_srt(&link).await.unwrap();
}
