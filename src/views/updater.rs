use crate::views::clear_term;

use super::fzf_selector::{FzfSelector, SelectType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use walkdir::WalkDir;

use super::menu::MenuOptions;

type JsonFormat = HashMap<String, String>;

pub struct Updater {
    mv_dir: String,
    audio_dir: String,
    audio_video: Arc<RefCell<JsonFormat>>,
    mvs_found: Option<Vec<String>>,
    audio_found: Option<Vec<String>>,
    selected_mv: Option<String>,
}

impl Updater {
    pub fn new(mv_dir: String, audio_dir: String, audio_video: Arc<RefCell<JsonFormat>>) -> Self {
        Self {
            mv_dir,
            audio_dir,
            audio_video,
            mvs_found: None,
            audio_found: None,
            selected_mv: None,
        }
    }

    pub fn start(&mut self) -> MenuOptions {
        loop {
            if self.mvs_found.is_none() {
                self.scan_mvs();
            }
            if self.mvs_found.as_ref().unwrap().is_empty() {
                return MenuOptions::MVSelector;
            }
            if self.audio_found.is_none() {
                println!("Scanning audio directory...");
                self.scan_audio();
            }
            clear_term("Select an MV to update")
                .unwrap_or_else(|e| eprintln!("Couldn't clear terminal: {}", e));
            let audio_list = self.audio_found.as_ref().unwrap();
            assert!(!audio_list.is_empty(), "No audio files found");
            let mv_list = self.mvs_found.as_ref().unwrap();
            let fzf_view = FzfSelector::new(
                Some(mv_list.clone()),
                Some(vec!["[[Back]]".to_owned()]),
                None,
            );
            self.selected_mv = None;
            self.selected_mv = Some(fzf_view.fzf_select(SelectType::Single));
            if self.selected_mv.as_ref().unwrap() == "[[Back]]" {
                return MenuOptions::MVSelector;
            }
            let list_audios = ListAudios::new(audio_list.clone());
            let selected_audio = list_audios.start();
            self.update_entry(
                self.selected_mv.as_ref().unwrap().to_owned(),
                selected_audio,
            );
        }
    }

    fn update_entry(&mut self, selected_mv: String, selected_audio: Option<String>) {
        if let Some(audio) = selected_audio {
            self.audio_video
                .as_ref()
                .borrow_mut()
                .insert(selected_mv.to_owned(), audio);
            assert!(
                self.audio_video
                    .as_ref()
                    .borrow_mut()
                    .contains_key(&selected_mv),
                "Failed to update entry"
            );
            self.mvs_found
                .as_mut()
                .unwrap()
                .retain(|mv| mv != &selected_mv);
            self.selected_mv = None;
        }
    }

    /// Scan the mv_dir for videos that are not in the audio_video json file
    /// and add them to the mvs_found list
    ///
    /// # Panics
    /// Panics if the mv_dir does not exists
    /// Panics if the mv_dir is not a directory
    fn scan_mvs(&mut self) {
        let mv_path = Path::new(&self.mv_dir);
        assert!(
            mv_path.exists(),
            "{}",
            format!("{} does not exist", mv_path.display())
        );
        assert!(
            mv_path.is_dir(),
            "{}",
            format!("{} is not a directory", mv_path.display())
        );
        let video_exts = [
            "mp4", "mkv", "avi", "webm", "ts", "flv", "wmv", "mov", "mpg", "mpeg",
        ];
        self.mvs_found = Some(
            mv_path
                .read_dir()
                .unwrap()
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry
                        .path()
                        .extension()
                        .map(|ext| video_exts.contains(&ext.to_str().unwrap()))
                        .unwrap_or(false)
                })
                .filter(|entry| {
                    !self
                        .audio_video
                        .borrow_mut()
                        .contains_key(entry.path().to_str().unwrap())
                })
                .map(|entry| entry.path().to_str().unwrap().to_string())
                .collect(),
        );
    }

    /// Scan the audio_dir recursively for audio files and add them to the audio_found list
    ///
    /// # Panics
    /// Panics if the audio_dir does not exists
    /// Panics if the audio_dir is not a directory
    fn scan_audio(&mut self) {
        let audio_path = Path::new(&self.audio_dir);
        assert!(
            audio_path.exists(),
            "{}",
            format!("{} does not exist", audio_path.display())
        );
        assert!(
            audio_path.is_dir(),
            "{}",
            format!("{} is not a directory", audio_path.display())
        );
        let walk_dir = WalkDir::new(audio_path).into_iter();
        let audio_exts = ["mp3", "wav", "ogg", "flac", "m4a", "aac"];
        self.audio_found = Some(
            walk_dir
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry
                        .path()
                        .extension()
                        .map(|ext| audio_exts.contains(&ext.to_str().unwrap()))
                        .unwrap_or(false)
                })
                .map(|entry| entry.path().to_str().unwrap().to_string())
                .collect(),
        );
    }
}

struct ListAudios {
    audio_list: Vec<String>,
}

impl ListAudios {
    pub fn new(audio_found: Vec<String>) -> Self {
        Self {
            audio_list: audio_found,
        }
    }

    pub fn start(&self) -> Option<String> {
        let fzf_view = FzfSelector::new(
            Some(self.audio_list.clone()),
            Some(vec!["[[Back]]".to_owned()]),
            None,
        );
        let selected_audio = fzf_view.fzf_select(SelectType::Single);
        if selected_audio.is_empty() {
            return None;
        }
        match selected_audio.as_str() {
            "[[Back]]" => None,
            _ => Some(selected_audio),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;
    use std::path::Path;
    use std::fs;

    fn create_file(parent: &Path, file: &Path) -> Result<(), std::io::Error> {
        fs::create_dir_all(parent)?;
        fs::File::create(file)?;
        Ok(())
    }

    #[test]
    fn test_scan_mvs() {
        let temp_dir = TempDir::new("test_scan_mvs").unwrap();
        let mv_dir = temp_dir.path().join("mv_dir");
        let video_file = mv_dir.join("video.mp4");
        create_file(&mv_dir, &video_file).unwrap();
        let mut updater = Updater::new(
            mv_dir.to_str().unwrap().to_string(),
            "".to_string(),
            Arc::new(RefCell::new(HashMap::new())),
        );
        updater.scan_mvs();
        assert_eq!(updater.mvs_found.as_ref().unwrap().len(), 1);
        assert_eq!(
            updater.mvs_found.as_ref().unwrap()[0],
            video_file.to_str().unwrap()
        );
        updater
            .audio_video
            .borrow_mut()
            .insert(video_file.to_str().unwrap().to_string(), "".to_string());
        updater.scan_mvs();
        assert_eq!(updater.mvs_found.as_ref().unwrap().len(), 0);
    }

    #[test]
    fn test_scan_audio() {
        let temp_dir = TempDir::new("test_scan_audio").unwrap();
        let audio_dir = temp_dir.path().join("audio_dir");
        let sub_dir_1 = audio_dir.join("sub_dir_1").join("sub_sub_dir_1");
        let audio_file = sub_dir_1.join("audio.mp3");
        let sub_dir_2 = audio_dir.join("sub_dir_2").join("sub_sub_dir_2");
        let audio_file_2 = sub_dir_2.join("audio_2.flac");
        let _not_audio_file = sub_dir_2.join("not_audio.txt");
        create_file(&sub_dir_1, &audio_file).unwrap();
        create_file(&sub_dir_2, &audio_file_2).unwrap();
        let mut updater = Updater::new(
            "".to_string(),
            audio_dir.to_str().unwrap().to_string(),
            Arc::new(RefCell::new(HashMap::new())),
        );
        updater.scan_audio();
        assert_eq!(updater.audio_found.as_ref().unwrap().len(), 2);
        assert!(updater
            .audio_found
            .as_ref()
            .unwrap()
            .contains(&audio_file.to_str().unwrap().to_string()));
        assert!(updater
            .audio_found
            .as_ref()
            .unwrap()
            .contains(&audio_file_2.to_str().unwrap().to_string()));
    }

    #[test]
    fn test_update_entry() {
        let rc = Arc::new(RefCell::new(HashMap::new()));
        let mut updater = Updater::new("".to_string(), "".to_string(), rc.clone());
        updater.audio_found = Some(vec!["audio.mp3".to_string()]);
        updater.mvs_found = Some(vec!["mv_0.mp4".to_string(), "mv_1.mp4".to_string()]);
        assert_eq!(rc.borrow().len(), 0);
        updater.update_entry("mv_0.mp4".to_string(), Some("audio.mp3".to_string()));
        assert_eq!(rc.borrow().len(), 1);
        assert_eq!(
            rc.borrow().get("mv_0.mp4").unwrap().to_string(),
            "audio.mp3".to_string()
        );
    }
}
