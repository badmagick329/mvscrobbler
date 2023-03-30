#![allow(unused_imports, dead_code, unused_variables, unused_must_use)]
pub mod avmod;
pub mod media_player;
pub mod views;

use std::cell::RefCell;
use std::io::{stdout, Write};

use avmod::AudioVideoData;
use crossterm::{cursor, terminal, QueueableCommand};
use views::fzf_selector::FzfSelector;
use views::menu::{MainMenu, MenuOptions};
use views::mv_selector::{FilterTypes, MVSelector};
use views::updater::Updater;

const AVINFO: &str = "/media/badmagick/HDD/Projects/rust_mvplayer/avinfo.json";
const VIDEO_DIR: &str = "/media/badmagick/HDD/Music/MVs/";
const AUDIO_DIR: &str = "/media/badmagick/HDD/Music/Library/";
// const AVINFO: &str = "F:\\Projects\\rust_mvplayer\\avinfo.json";
// const VPATH_PREFIX: &str = "F:\\Music\\MVs\\";
// const APATH_PREFIX: &str = "F:\\";

pub async fn run() {
    let mut avd = AudioVideoData::new(AVINFO, VIDEO_DIR.to_string(), AUDIO_DIR.to_string());
    avd.load_data();
    let audio_video = RefCell::new(avd.audio_video.clone());
    let mut mv_selector = MVSelector::new(avd);
    let mut selected_opt: MenuOptions = MenuOptions::MVSelector;
    loop {
        match selected_opt {
            MenuOptions::Quit => {
                println!("Exiting...");
                break;
            }
            MenuOptions::MainMenu => {
                selected_opt = MainMenu::default().start();
            }
            MenuOptions::MVSelector => {
                selected_opt = mv_selector.start().await;
            }
            MenuOptions::ToggleLive => {
                selected_opt = mv_selector.toggle_filter(FilterTypes::Live);
            }
            MenuOptions::ToggleMVs => {
                selected_opt = mv_selector.toggle_filter(FilterTypes::MVs);
            }
            MenuOptions::Update => {
                let mut updater = Updater::new(
                    AVINFO,
                    VIDEO_DIR.to_string(),
                    AUDIO_DIR.to_string(),
                    audio_video.clone(),
                );
                selected_opt = updater.start();
                mv_selector.avd.update_json(audio_video.borrow().clone());
            }
        }
    }
}
