#![allow(unused_imports, dead_code, unused_variables, unused_must_use)]
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::sync;
use tokio::task::JoinHandle;
use tokio::time;

#[derive(Default)]
pub struct MediaPlayer {
    audio_tx: Option<Arc<sync::mpsc::Sender<usize>>>,
    video_tx: Option<Arc<sync::mpsc::Sender<usize>>>,
    video_cmd: String,
    audio_cmd: String,
}

impl MediaPlayer {
    pub fn new(video_cmd: String, audio_cmd: String) -> Self {
        Self {
            video_cmd,
            audio_cmd,
            ..Default::default()
        }
    }
    pub async fn play_media(&mut self, audio_path: String, video_path: String) {
        if self.audio_tx.is_some() {
            self.audio_tx.as_ref().unwrap().send(1).await;
        }
        if self.video_tx.is_some() {
            self.video_tx.as_ref().unwrap().send(1).await;
        }
        let (audio_tx, mut audio_rx) = sync::mpsc::channel::<usize>(1);
        let (video_tx, mut video_rx) = sync::mpsc::channel::<usize>(1);
        self.audio_tx = Some(Arc::new(audio_tx));
        self.video_tx = Some(Arc::new(video_tx));
        let audio_cmd = self.audio_cmd.clone();
        let video_cmd = self.video_cmd.clone();
        tokio::spawn(async move {
            let audio_split = audio_cmd.split(",");
            let mut child = Command::new(audio_split.clone().next().unwrap())
                .args(audio_split.skip(1))
                .arg(audio_path)
                .spawn()
                .expect("failed to spawn video player task");
            while child.try_wait().unwrap().is_none() {
                time::sleep(time::Duration::from_millis(1000)).await;
                if audio_rx.try_recv().is_ok() {
                    child.kill().await;
                    break;
                }
            }
        });
        tokio::spawn(async move {
            let video_split = video_cmd.split(",");
            let mut child = Command::new(video_split.clone().next().unwrap())
                .args(video_split.skip(1))
                .arg(video_path)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
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
