#![allow(unused_imports, dead_code, unused_variables, unused_must_use)]
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::slice::Iter;
use std::str;

use super::menu::MenuOptions;

// Updater: Get list of videos that need to be updated. If none, show message and offer back as
// option. If back, return MVSelector
// has these states: Unscanned, Scanner, MVList, AudioList
// Unscanned: transition into Scanner
// Scanner: Show progress bar, scan a folder, update progress. Repeat until no more folders.
// Transition into MVList
// MVList: Show list of MVs, and back option. If back, return MVSelector. If MV Selected,
// transition into AudioList
// AudioList: Show list of audio files, and back option. If back, transition into MVList. If audio
// selected, link the MV and audio file. Remove MV from mv list. If no more MVs, return to
// MVSelector. Else return to MVList

type JsonFormat = HashMap<String, String>;

struct UpdateData {
    mv_dir: String,
    audio_dir: String,
    audio_video: JsonFormat,
    mvs_found: Option<Vec<String>>,
    audio_found: Option<Vec<String>>,
}
//
// struct Unscanned {
//     data: UpdateData,
// }
//
// impl Unscanned {
//     fn new() -> Self {
//         Self {}
//     }
//
//     fn
// }

pub struct MVList {
    update_data: UpdateData,
}

impl MVList {
    pub fn new(mv_dir: String, audio_dir: String, audio_video: JsonFormat) -> Self {
        Self {
            update_data: UpdateData {
                mv_dir,
                audio_dir,
                audio_video,
                mvs_found: None,
                audio_found: None,
            },
        }
    }

    pub fn start(&mut self) -> MenuOptions {
        if self.update_data.mvs_found.is_none() {
            self.scan_mvs();
        }
        if self.update_data.mvs_found.as_ref().unwrap().len() == 0 {
            println!("No more videos to update");
            return MenuOptions::MVSelector;
        }
        if self.update_data.audio_found.is_none() {
            self.scan_audio();
        }
        todo!()
    }

    fn scan_mvs(&mut self) {
        let mv_path = Path::new(&self.update_data.mv_dir);
        assert!(
            mv_path.exists(),
            "{}",
            format!("{} does not exist", mv_path.display())
        );
        let video_exts = [
            "mp4", "mkv", "avi", "webm", "ts", "flv", "wmv", "mov", "mpg", "mpeg",
        ];
        self.update_data.mvs_found = Some(
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
                        .update_data
                        .audio_video
                        .contains_key(entry.path().to_str().unwrap())
                })
                .map(|entry| entry.path().to_str().unwrap().to_string())
                .collect(),
        );
    }

    fn scan_audio(&mut self) {
        let audio_path = Path::new(&self.update_data.audio_dir);
        assert!(
            audio_path.exists(),
            "{}",
            format!("{} does not exist", audio_path.display())
        );
        let audio_exts = ["mp3", "wav", "ogg", "flac", "m4a", "aac"];
        self.update_data.audio_found = Some(
            audio_path
                .read_dir()
                .unwrap()
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
