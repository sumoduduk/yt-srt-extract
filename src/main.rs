mod yt_caption;
use std::env::args;
use yt_caption::extract_srt;

#[tokio::main]
async fn main() {
    let mut args = args();

    args.next();

    let link = args.next().expect("Need to provide link in arg 2");
    let format_res = args.next().expect("Need to format text or json in arg 3");

    extract_srt(&link, &format_res).await.unwrap();
}
