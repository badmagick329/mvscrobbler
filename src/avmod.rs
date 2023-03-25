#![allow(unused_imports, dead_code, unused_variables, unused_must_use)]
use super::media_player::MediaPlayer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fmt, fs};

#[derive(Serialize, Deserialize)]
pub struct AudioVideo {
    audio_path: String,
    video_path: String,
}

impl AudioVideo {
    pub fn new(audio_path: String, video_path: String) -> Self {
        Self {
            audio_path,
            video_path,
        }
    }
}

impl Clone for AudioVideo {
    fn clone(&self) -> Self {
        Self {
            audio_path: self.audio_path.clone(),
            video_path: self.video_path.clone(),
        }
    }
}

impl fmt::Display for AudioVideo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Audio: {}, Video: {}", self.audio_path, self.video_path)
    }
}

pub struct AudioVideoData {
    pub data_file: String,
    pub audio_video: Vec<AudioVideo>,
    player: MediaPlayer,
}

type JsonFormat = HashMap<String, String>;

impl AudioVideoData {
    pub fn new(data_file: &str) -> Self {
        Self {
            data_file: data_file.to_string(),
            audio_video: Vec::new(),
            player: MediaPlayer::new(),
        }
    }

    pub async fn play_media(&mut self, av: &AudioVideo) {
        self.player
            .play_media(av.audio_path.to_owned(), av.video_path.to_owned())
            .await;
    }

    pub fn load_data(&mut self) {
        let data = fs::read_to_string(&self.data_file).expect("Unable to read data file");

        let read_data =
            serde_json::from_str::<JsonFormat>(&data).expect("Unable to parse data file");
        for (video_path, audio_path) in read_data {
            self.audio_video.push(AudioVideo::new(
                format!("/media/badmagick/HDD/{audio_path}"),
                format!("/media/badmagick/HDD/Music/MVs/{video_path}"),
            ));
        }
    }
}

// use serde::{Deserialize, Serialize};
// use std::path::{Component, Path, PathBuf};
// use std::{fs, process};
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct YamlData {
//     pub source: String,
//     pub folders: Vec<String>,
//     pub target: String,
// }
// /// Read the yaml file which contains the sourch dir, the dirs to be backed up and the target dir
// pub fn read_yaml(file_name: &str) -> Result<YamlData> {
//     let file_data = fs::read_to_string(file_name)
//         .wrap_err(format!("Error while reading the file {file_name}"))?;
//     let yaml: YamlData = serde_yaml::from_str(file_data.as_str())?;
//     validate_yaml(&yaml)?;
//     Ok(yaml)
// }

