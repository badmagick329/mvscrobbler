#![allow(dead_code, unused_mut)]
use super::media_player::MediaPlayer;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};

type JsonFormat = HashMap<String, String>;

pub struct AudioVideoData {
    vpath_prefix: String,
    apath_prefix: String,
    pub data_file: String,
    pub audio_video: JsonFormat,
    video_list: Option<Vec<String>>,
    player: MediaPlayer,
}

impl AudioVideoData {
    pub fn new(data_file: &str, apath_prefix: &str, vpath_prefix: &str) -> Self {
        Self {
            vpath_prefix: vpath_prefix.to_string(),
            apath_prefix: apath_prefix.to_string(),
            data_file: data_file.to_string(),
            audio_video: HashMap::new(),
            video_list: None,
            player: MediaPlayer::default(),
        }
    }

    pub async fn play_media(&mut self, video_name: &str) {
        let video_path = self.to_video_path(video_name);
        self.player
            .play_media(
                self.audio_video.get(&video_path).unwrap().to_owned(),
                video_path.to_owned(),
            )
            .await;
    }

    pub fn load_data(&mut self) {
        let data = fs::read_to_string(&self.data_file).expect("Unable to read data file");
        let read_data =
            serde_json::from_str::<JsonFormat>(&data).expect("Unable to parse data file");
        for (video_path, audio_path) in read_data {
            self.audio_video.insert(
                format!("{}{}", self.vpath_prefix, video_path),
                format!("{}{}", self.apath_prefix, audio_path),
            );
        }
    }

    pub fn save_data(&mut self) {
        let mut save_formatted = HashMap::new();
        for (video_path, audio_path) in &self.audio_video {
            save_formatted.insert(
                video_path.replace(&self.vpath_prefix, ""),
                audio_path.replace(&self.apath_prefix, ""),
            );
        }
        let data = serde_json::to_string_pretty(&save_formatted).unwrap();
        let mut file = fs::File::create(&self.data_file).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }

    pub fn list_videos(&mut self) -> Vec<String> {
        if self.video_list.is_none() {
            let mut vlist = self
                .audio_video
                .clone()
                .keys()
                .map(|k| k.to_string().replace(&self.vpath_prefix, ""))
                .collect::<Vec<String>>();
            vlist.sort();
            self.video_list = Some(vlist);
        }
        self.video_list.clone().unwrap()
    }

    fn to_video_path(&self, video_name: &str) -> String {
        format!("{}{}", self.vpath_prefix, video_name)
    }
    fn to_audio_path(&self, audio_name: &str) -> String {
        format!("{}{}", self.apath_prefix, audio_name)
    }

    pub fn get_json(&self) -> JsonFormat {
        self.audio_video.clone()
    }
    pub fn update_json(&mut self, json: JsonFormat) {
        self.audio_video = json;
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_save_data() {
        let temp_dir = TempDir::new("test_save_data").unwrap();
        let data_file = temp_dir.path().join("data.json");
        let mut av_data = AudioVideoData::new(data_file.to_str().unwrap(), "audio/", "video/");
        av_data
            .audio_video
            .insert("video/1.mp4".to_string(), "audio/1.mp3".to_string());
        av_data
            .audio_video
            .insert("video/2.mp4".to_string(), "audio/2.mp3".to_string());
        av_data.save_data();
        let data = fs::read_to_string(&data_file).expect("Unable to read data file");
        let read_data =
            serde_json::from_str::<JsonFormat>(&data).expect("Unable to parse data file");
        assert_eq!(read_data.len(), 2);
        assert_eq!(read_data.get("1.mp4").unwrap(), "1.mp3");
        assert_eq!(read_data.get("2.mp4").unwrap(), "2.mp3");
    }

    #[test]
    fn test_load_data() {
        let temp_dir = TempDir::new("test_load_data").unwrap();
        let data_file = temp_dir.path().join("data.json");
        let mut av_data = AudioVideoData::new(data_file.to_str().unwrap(), "audio/", "video/");
        av_data
            .audio_video
            .insert("video/1.mp4".to_string(), "audio/1.mp3".to_string());
        av_data
            .audio_video
            .insert("video/2.mp4".to_string(), "audio/2.mp3".to_string());
        av_data.save_data();
        let mut av_data = AudioVideoData::new(data_file.to_str().unwrap(), "audio/", "video/");
        av_data.load_data();
        assert_eq!(av_data.audio_video.len(), 2);
        assert_eq!(av_data.audio_video.get("video/1.mp4").unwrap(), "audio/1.mp3");
        assert_eq!(av_data.audio_video.get("video/2.mp4").unwrap(), "audio/2.mp3");
    }
}
