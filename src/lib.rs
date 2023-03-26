#![allow(unused_imports, dead_code, unused_variables, unused_must_use)]
pub mod avmod;
pub mod media_player;
pub mod views;

use std::io::{stdout, Write};

use avmod::AudioVideoData;
use crossterm::{cursor, terminal, QueueableCommand};
use views::{FzfSelector, MVSelector, MainMenu, ViewTypes};

const AVINFO: &str = "/media/badmagick/HDD/Projects/rust_mvplayer/avinfo.json";
const VPATH_PREFIX: &str = "/media/badmagick/HDD/Music/MVs/";
const APATH_PREFIX: &str = "/media/badmagick/HDD/";
// const AVINFO: &str = "F:\\Projects\\rust_mvplayer\\avinfo.json";
// const VPATH_PREFIX: &str = "F:\\Music\\MVs\\";
// const APATH_PREFIX: &str = "F:\\";

pub async fn run() {
    let mut avd = AudioVideoData::new(AVINFO, APATH_PREFIX, VPATH_PREFIX);
    avd.load_data();
    let mut mv_selector = MVSelector::new(avd);
    let mut view_type: ViewTypes = ViewTypes::MVSelector;
    loop {
        match view_type {
            ViewTypes::Quit => {
                println!("Exiting...");
                break;
            }
            ViewTypes::MainMenu => {
                view_type = MainMenu::new().start();
            }
            ViewTypes::MVSelector => {
                view_type = mv_selector.start().await;
            }
        }
    }
}
