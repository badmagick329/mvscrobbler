#![allow(unused_imports, dead_code, unused_variables, unused_must_use)]
pub mod avmod;
pub mod media_player;
pub mod views;
pub mod config;

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::sync::Arc;

use avmod::AudioVideoData;
use crossterm::{cursor, terminal, QueueableCommand};
use views::fzf_selector::FzfSelector;
use views::menu::{MainMenu, MenuOptions};
use views::mv_selector::{FilterTypes, MVSelector};
use views::updater::Updater;
use config::{Config, ConfigError};

// const AVINFO: &str = "/media/badmagick/HDD/Projects/rust_mvplayer/avinfo.json_test";
// const VIDEO_DIR: &str = "/media/badmagick/HDD/Music/test_mvs/";
// const AUDIO_DIR: &str = "/media/badmagick/HDD/Music/test_library/";
// const AVINFO: &str = "F:\\Projects\\rust_mvplayer\\avinfo.json";
// const VPATH_PREFIX: &str = "F:\\Music\\MVs\\";
// const APATH_PREFIX: &str = "F:\\";

type JsonFormat = HashMap<String, String>;

pub async fn run() {
    let config = Config::build("config.yml").unwrap();
    let audio_video = Arc::new(RefCell::new(HashMap::new()));
    let mut avd = AudioVideoData::new(
        config.data_file.as_str(),
        config.video_dir.to_string(),
        config.audio_dir.to_string(),
        audio_video.clone(),
        config.video_cmd.to_string(),
        config.audio_cmd.to_string(),
    );
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
            MenuOptions::Update => {
                let mut updater = Updater::new(
                    config.video_dir.to_string(),
                    config.audio_dir.to_string(),
                    audio_video.clone(),
                );
                selected_opt = updater.start();
                mv_selector.avd.save_data();
                mv_selector.avd.video_list = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use super::*;

    #[test]
    fn arc_refcell_test_1() {
        let hash_map = Arc::new(RefCell::new(HashMap::new()));
        let hash_map2 = hash_map.clone();

        hash_map.borrow_mut().insert("test", "test");
        hash_map2.borrow_mut().insert("test2", "test2");

        assert!(hash_map.borrow().contains_key("test"));
        assert!(hash_map.borrow().contains_key("test2"));

        assert!(hash_map2.borrow().contains_key("test"));
        assert!(hash_map2.borrow().contains_key("test2"));
    }

    #[test]
    fn arc_refcell_test_2() {
        let hash_map = Arc::new(RefCell::new(HashMap::new()));
        struct MyStruct {
            hmap: Arc<RefCell<HashMap<String, String>>>,
        }
        let strct = MyStruct { hmap: hash_map.clone() };
        strct.hmap.borrow_mut().insert("test2".to_owned(), "test2".to_owned());
        assert!(strct.hmap.borrow().contains_key("test2"));
    }
}
