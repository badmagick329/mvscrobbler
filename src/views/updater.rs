#![allow(unused_imports, dead_code, unused_variables, unused_must_use)]
use super::super::avmod::AudioVideoData;
use crossterm::{cursor, terminal, QueueableCommand};
use std::io::{stdout, Write};
use std::process::{Command, Stdio};
use std::slice::Iter;
use std::str;

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

// struct UpdateData {
//     mv_dir: String,
// }
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
