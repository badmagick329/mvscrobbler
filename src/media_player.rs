#![allow(unused_imports, dead_code, unused_variables, unused_must_use)]
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::sync;
use tokio::task::JoinHandle;
use tokio::time;

pub struct MediaPlayer {
    audio_tx: Option<Arc<sync::mpsc::Sender<u32>>>,
    video_tx: Option<Arc<sync::mpsc::Sender<u32>>>,
}

impl MediaPlayer {
    pub fn new() -> Self {
        Self {
            audio_tx: None,
            video_tx: None,
        }
    }

    pub async fn play_media(&mut self, audio_path: String, video_path: String) {
        if self.audio_tx.is_some() {
            self.audio_tx.as_ref().unwrap().send(1).await;
        }
        if self.video_tx.is_some() {
            self.video_tx.as_ref().unwrap().send(1).await;
        }
        let (audio_tx, mut audio_rx) = sync::mpsc::channel::<u32>(1);
        let (video_tx, mut video_rx) = sync::mpsc::channel::<u32>(1);
        self.audio_tx = Some(Arc::new(audio_tx));
        self.video_tx = Some(Arc::new(video_tx));

        tokio::spawn(async move {
            let mut child = Command::new("audacious")
                .arg(audio_path)
                .spawn()
                .expect("failed to spawn audio player task");
            while child.try_wait().unwrap().is_none() {
                time::sleep(time::Duration::from_millis(1000)).await;
                if audio_rx.try_recv().is_ok() {
                    child.kill().await;
                    break;
                }
            }
        });
        tokio::spawn(async move {
            let mut child = Command::new("mpv")
                .arg(video_path)
                .arg("--no-terminal")
                // .stdout(Stdio::null())
                .spawn()
                .expect("failed to spawn video player task");
            while child.try_wait().unwrap().is_none() {
                time::sleep(time::Duration::from_millis(1000)).await;
                if video_rx.try_recv().is_ok() {
                    child.kill().await;
                    break;
                }
            }
        });
    }
}
