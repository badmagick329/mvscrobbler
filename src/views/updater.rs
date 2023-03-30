#![allow(unused_imports, dead_code, unused_variables, unused_mut)]
use crate::views::clear_term;

use super::fzf_selector::FzfSelector;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::io::{stdout, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::slice::Iter;
use std::str;
use walkdir::WalkDir;

use super::menu::MenuOptions;

type JsonFormat = HashMap<String, String>;

pub struct Updater {
    data_file: String,
    mv_dir: String,
    audio_dir: String,
    audio_video: RefCell<JsonFormat>,
    mvs_found: Option<Vec<String>>,
    audio_found: Option<Vec<String>>,
    selected_mv: Option<String>,
}

impl Updater {
    pub fn new(
        data_file: &str,
        mv_dir: String,
        audio_dir: String,
        audio_video: RefCell<JsonFormat>,
    ) -> Self {
        Self {
            data_file: data_file.to_owned(),
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
            if self.mvs_found.as_ref().unwrap().len() == 0 {
                return MenuOptions::MVSelector;
            }
            if self.audio_found.is_none() {
                println!("Scanning audio directory...");
                self.scan_audio();
            }
            clear_term("Select an MV to update")
                .unwrap_or_else(|e| eprintln!("Couldn't clear terminal: {}", e));
            let audio_list = self.audio_found.as_ref().unwrap();
            assert!(audio_list.len() > 0, "No audio files found");
            let mv_list = self.mvs_found.as_ref().unwrap();
            let fzf_view = FzfSelector::new(
                Some(mv_list.clone()),
                Some(vec!["[[Back]]".to_owned()]),
                None,
            );
            self.selected_mv = None;
            self.selected_mv = Some(fzf_view.fzf_select());
            if self.selected_mv.as_ref().unwrap() == "[[Back]]" {
                return MenuOptions::MVSelector;
            }
            let list_audios = ListAudios::new(audio_list.clone());
            let selected_audio = list_audios.start();
            if let Some(audio) = selected_audio {
                self.audio_video
                    .borrow_mut()
                    .insert(self.selected_mv.as_ref().unwrap().to_owned(), audio);
                self.mvs_found
                    .as_mut()
                    .unwrap()
                    .retain(|mv| mv != self.selected_mv.as_ref().unwrap());
                self.selected_mv = None;
            }
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
        let selected_audio = fzf_view.fzf_select();
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

    fn create_file(parent: &Path, file: &Path) -> Result<(), std::io::Error> {
        if let Err(e) = fs::create_dir_all(&parent) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                return Err(e);
            }
        }
        if let Err(e) = fs::File::create(&file) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                return Err(e);
            }
        }
        Ok(())
    }

    #[test]
    fn test_scan_mvs() {
        let temp_dir = TempDir::new("test_scan_mvs").unwrap();
        let mv_dir = temp_dir.path().join("mv_dir");
        let video_file = mv_dir.join("video.mp4");
        create_file(&mv_dir, &video_file).unwrap();
        let mut updater = Updater::new(
            "",
            mv_dir.to_str().unwrap().to_string(),
            "".to_string(),
            RefCell::new(HashMap::new()),
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
        let not_audio_file = sub_dir_2.join("not_audio.txt");
        create_file(&sub_dir_1, &audio_file).unwrap();
        create_file(&sub_dir_2, &audio_file_2).unwrap();
        let mut updater = Updater::new(
            "",
            "".to_string(),
            audio_dir.to_str().unwrap().to_string(),
            RefCell::new(HashMap::new()),
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
}
