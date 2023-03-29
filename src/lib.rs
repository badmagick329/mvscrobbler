#![allow(unused_imports, dead_code, unused_variables, unused_must_use)]
pub mod avmod;
pub mod media_player;
pub mod views;

use std::io::{stdout, Write};

use avmod::AudioVideoData;
use crossterm::{cursor, terminal, QueueableCommand};
use views::updater;
use views::fzf_selector::FzfSelector;
use views::menu::{MenuOptions, MainMenu};
use views::mv_selector::{MVSelector, FilterTypes};

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
        }
    }
}
