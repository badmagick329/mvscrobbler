#![allow(unused_imports, dead_code, unused_variables, unused_must_use)]
use rust_mvplayer::avmod::{AudioVideo, AudioVideoData};
use rust_mvplayer::media_player::MediaPlayer;
use rust_mvplayer::run;
use tokio::process::Command;
use tokio::time;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    run().await;
}
