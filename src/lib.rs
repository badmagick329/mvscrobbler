#![allow(unused_imports, dead_code, unused_variables, unused_must_use)]
pub mod views;
pub mod avmod;
pub mod media_player;

use avmod::{AudioVideo, AudioVideoData};
use tokio::time;
use views::FzfMenu;

pub async fn run() {
    let mut avd = AudioVideoData::new("/media/badmagick/HDD/Projects/rust_mvplayer/avinfo.json");
    avd.load_data();
    for (i, av) in avd.audio_video.clone().iter().take(2).enumerate() {
        println!("{}", av);
        println!("Playing media");
        avd.play_media(&av).await;
        println!("Waiting 5 seconds");
        time::sleep(time::Duration::from_secs(5)).await;
    }
}
