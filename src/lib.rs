pub mod avmod;
pub mod config;
pub mod media_player;
pub mod views;

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use avmod::{AudioVideoData, Sorting};
use config::Config;
use views::menu::{MainMenu, MenuOptions};
use views::mv_selector::{FilterTypes, MVSelector};
use views::search_filter::SearchFilters;
use views::updater::Updater;

// const AVINFO: &str = "/media/badmagick/HDD/Projects/rust_mvplayer/avinfo.json_test";
// const VIDEO_DIR: &str = "/media/badmagick/HDD/Music/test_mvs/";
// const AUDIO_DIR: &str = "/media/badmagick/HDD/Music/test_library/";
// const AVINFO: &str = "F:\\Projects\\rust_mvplayer\\avinfo.json";
// const VPATH_PREFIX: &str = "F:\\Music\\MVs\\";
// const APATH_PREFIX: &str = "F:\\";

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
            MenuOptions::SortAsc => {
                match mv_selector.avd.sorting {
                    Sorting::Ascending => {}
                    Sorting::Descending => {
                        mv_selector.avd.sorting = Sorting::Ascending;
                        mv_selector.avd.video_list = None;
                    }
                    Sorting::Mtime => {
                        mv_selector.avd.sorting = Sorting::Ascending;
                        mv_selector.avd.video_list = None;
                    }
                }
                selected_opt = MenuOptions::MVSelector;
            }
            MenuOptions::SortDesc => {
                match mv_selector.avd.sorting {
                    Sorting::Ascending => {
                        mv_selector.avd.sorting = Sorting::Descending;
                        mv_selector.avd.video_list = None;
                    }
                    Sorting::Descending => {}
                    Sorting::Mtime => {
                        mv_selector.avd.sorting = Sorting::Descending;
                        mv_selector.avd.video_list = None;
                    }
                }
                selected_opt = MenuOptions::MVSelector;
            }
            MenuOptions::SortMtime => {
                match mv_selector.avd.sorting {
                    Sorting::Ascending => {
                        mv_selector.avd.sorting = Sorting::Mtime;
                        mv_selector.avd.video_list = None;
                    }
                    Sorting::Descending => {
                        mv_selector.avd.sorting = Sorting::Mtime;
                        mv_selector.avd.video_list = None;
                    }
                    Sorting::Mtime => {}
                }
                selected_opt = MenuOptions::MVSelector;
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
            MenuOptions::Random => {
                selected_opt = mv_selector.play_random().await;
            }
            MenuOptions::SearchFilter => {
                match &mv_selector.avd.search_filtered_list {
                    None => {
                        let av = mv_selector.filtered_list();
                        let mut search_filter = SearchFilters::new(av);
                        let selected_videos = search_filter.start();
                        mv_selector.set_search_filters(Some(selected_videos.to_owned()));
                    }
                    Some(_) => mv_selector.set_search_filters(None),
                }
                selected_opt = MenuOptions::MVSelector;
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
        let strct = MyStruct { hmap: hash_map };
        strct
            .hmap
            .borrow_mut()
            .insert("test2".to_owned(), "test2".to_owned());
        assert!(strct.hmap.borrow().contains_key("test2"));
    }
}
