#![allow(dead_code, unused_mut)]
use super::media_player::MediaPlayer;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

type JsonFormat = HashMap<String, String>;
#[derive(Debug, Clone, Copy,PartialEq)]
pub enum Sorting {
    Ascending,
    Descending,
    Mtime,
}

/// Used to store the data for the media files
/// # Fields
/// * `data_file`: The path to the json file that stores the data_file
/// * `video_dir`: The path to the directory that contains the video files. All videos are assumed to
/// be in this directory
/// * `audio_dir`: The path to the directory that contains the audio files. All audio files are
/// assumed to be in this directory
/// * `audio_video`: The data that is stored in the json file. The key is the video file name, and
/// the value is the audio file path inside the audio directory
/// * `video_list`: The list of video file names without the full path
/// * `player`: The media player that is used to play the media files
pub struct AudioVideoData {
    pub data_file: String,
    pub video_dir: String,
    pub audio_dir: String,
    pub audio_video: Arc<RefCell<JsonFormat>>,
    pub video_list: Option<Vec<String>>,
    pub sorting: Sorting,
    player: MediaPlayer,
}

impl AudioVideoData {
    pub fn new(
        data_file: &str,
        video_dir: String,
        audio_dir: String,
        audio_video: Arc<RefCell<JsonFormat>>,
        video_cmd: String,
        audio_cmd: String,
    ) -> Self {
        Self {
            data_file: data_file.to_string(),
            video_dir,
            audio_dir,
            audio_video,
            video_list: None,
            sorting: Sorting::Descending,
            player: MediaPlayer::new(video_cmd, audio_cmd),
        }
    }

    pub async fn play_media(&mut self, video_name: &str) {
        let prefix = Path::new(&self.video_dir);
        let binding = prefix.join(video_name);
        let video_path = binding.to_str().unwrap();
        self.player
            .play_media(
                self.audio_video
                    .borrow()
                    .get(video_path)
                    .unwrap()
                    .to_owned(),
                video_path.to_owned(),
            )
            .await;
    }

    pub fn load_data(&mut self) {
        let data = fs::read_to_string(&self.data_file).expect("Unable to read data file");
        let read_data =
            serde_json::from_str::<JsonFormat>(&data).expect("Unable to parse data file");
        let mut update_save = false;
        for (video_path, audio_path) in read_data {
            let full_vpath = Path::new(&self.video_dir).join(&video_path);
            let full_apath = Path::new(&self.audio_dir).join(&audio_path);
            if !full_vpath.exists() || !full_apath.exists() {
                update_save = true;
                continue;
            }
            self.audio_video.borrow_mut().insert(
                full_vpath.to_str().unwrap().to_string(),
                full_apath.to_str().unwrap().to_string(),
            );
        }
        if update_save {
            self.save_data();
        }
    }

    pub fn save_data(&mut self) {
        let to_save = self.audio_video.borrow().clone();
        let data = serde_json::to_string_pretty(&to_save).unwrap();
        let mut file = fs::File::create(&self.data_file).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }

    pub fn list_videos(&mut self) -> Vec<String> {
        if self.video_list.is_some() {
            return self.video_list.clone().unwrap();
        }
        let mut vlist = self
            .audio_video
            .borrow()
            .clone()
            .keys()
            .map(|k| {
                k.to_string()
                    .trim_start_matches(&self.video_dir)
                    .trim_start_matches('/')
                    .trim_start_matches('\\')
                    .to_string()
            })
            .collect::<Vec<String>>();
        match self.sorting {
            Sorting::Ascending => vlist.sort(),
            Sorting::Descending => vlist.sort_by(|a, b| b.cmp(a)),
            Sorting::Mtime => {
                let mut vlist2 = vlist
                    .iter()
                    .map(|k| {
                        let full_path = Path::new(&self.video_dir).join(k);
                        let metadata = fs::metadata(full_path).unwrap();
                        let mtime = metadata.modified().unwrap();
                        (k.to_string(), mtime)
                    })
                    .collect::<Vec<(String, std::time::SystemTime)>>();
                vlist2.sort_by(|a, b| b.1.cmp(&a.1));
                vlist = vlist2.iter().map(|k| k.0.to_string()).collect();
            }
        }
        self.video_list = Some(vlist.clone());
        vlist
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_save_data() {
        let temp_dir = TempDir::new("test_save_data").unwrap();
        let data_file = temp_dir.path().join("data.json");
        let rc = Arc::new(RefCell::new(HashMap::new()));
        let mut av_data = AudioVideoData::new(
            data_file.to_str().unwrap(),
            "video".to_string(),
            "audio".to_string(),
            rc,
            "".to_string(),
            "".to_string(),
        );
        // let mut borrowed = av_data.audio_video.deref().borrow_mut();
        // let mut borrowed = av_data.audio_video.as_ref().borrow_mut();
        av_data
            .audio_video
            .as_ref()
            .borrow_mut()
            .insert("video/1.mp4".to_string(), "audio/1.mp3".to_string());
        av_data
            .audio_video
            .as_ref()
            .borrow_mut()
            .insert("video/2.mp4".to_string(), "audio/2.mp3".to_string());
        av_data.save_data();
        let data = fs::read_to_string(&data_file).unwrap();
        let read_data = serde_json::from_str::<JsonFormat>(&data).unwrap();
        assert_eq!(read_data.len(), 2);
    }

    fn create_file(parent: &Path, file: &Path) -> Result<(), std::io::Error> {
        if let Err(e) = fs::create_dir(parent) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                return Err(e);
            }
        }
        if let Err(e) = fs::File::create(file) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                return Err(e);
            }
        }
        Ok(())
    }

    #[test]
    fn test_load_data() {
        let temp_dir = TempDir::new("test_load_data").unwrap();
        let data_file = temp_dir.path().join("data.json");
        let video_dir = temp_dir.path().join("video");
        let audio_dir = temp_dir.path().join("audio");
        let video_file1 = video_dir.join("1.mp4");
        let audio_file1 = audio_dir.join("1.mp3");
        create_file(&video_dir, &video_file1).unwrap();
        create_file(&audio_dir, &audio_file1).unwrap();
        let rc = Arc::new(RefCell::new(HashMap::new()));
        let mut av_data = AudioVideoData::new(
            data_file.to_str().unwrap(),
            video_dir.to_str().unwrap().to_string(),
            audio_dir.to_str().unwrap().to_string(),
            rc,
            "".to_string(),
            "".to_string(),
        );
        av_data.audio_video.as_ref().borrow_mut().insert(
            video_file1.to_str().unwrap().to_owned(),
            audio_file1.to_str().unwrap().to_owned(),
        );
        av_data.save_data();
        av_data.audio_video.as_ref().borrow_mut().clear();
        av_data.load_data();
        assert_eq!(av_data.audio_video.borrow().len(), 1);
    }

    #[test]
    fn test_load_data_non_existent() {
        let temp_dir = TempDir::new("test_load_data").unwrap();
        let data_file = temp_dir.path().join("data.json");
        let video_dir = temp_dir.path().join("video");
        let audio_dir = temp_dir.path().join("audio");
        let video_file1 = video_dir.join("1.mp4");
        let audio_file1 = audio_dir.join("1.mp3");
        if video_dir.exists() {
            fs::remove_dir_all(&video_dir).unwrap();
        }
        if audio_dir.exists() {
            fs::remove_dir_all(&audio_dir).unwrap();
        }
        let rc = Arc::new(RefCell::new(HashMap::new()));
        let mut av_data = AudioVideoData::new(
            data_file.to_str().unwrap(),
            video_dir.to_str().unwrap().to_string(),
            audio_dir.to_str().unwrap().to_string(),
            rc,
            "".to_string(),
            "".to_string(),
        );
        av_data.audio_video.as_ref().borrow_mut().insert(
            video_file1.to_str().unwrap().to_owned(),
            audio_file1.to_str().unwrap().to_owned(),
        );
        av_data.save_data();
        av_data.audio_video.as_ref().borrow_mut().clear();
        av_data.load_data();
        assert_eq!(av_data.audio_video.borrow().len(), 0);
    }
}
