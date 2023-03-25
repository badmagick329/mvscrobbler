#![allow(unused_imports, dead_code, unused_variables, unused_must_use)]
use std::io::{stdout, Write};
use std::process::{Child, Command, Stdio};
use std::str;

pub struct FzfMenu {
    height: String,
    inputs: Vec<String>,
    other_options: Vec<String>,
}

impl FzfMenu {
    pub fn new(
        inputs: Vec<String>,
        other_options: Option<Vec<String>>,
        height: Option<String>,
    ) -> FzfMenu {
        Self {
            height: height.unwrap_or("20%".to_string()),
            inputs,
            other_options: other_options.unwrap_or(Vec::new()),
        }
    }

    pub fn fzf_select(self) -> String {
        let mut child = Command::new("fzf")
            .args(&["--height", self.height.as_str(), "--reverse"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        let mut fzf_in = String::new();
        for input in self.inputs.iter() {
            fzf_in.push_str(&input);
            fzf_in.push('\n');
        }
        for option in self.other_options.iter() {
            fzf_in.push_str(format!("[{}]", option).as_str());
            fzf_in.push('\n');
        }
        stdin
            .write_all(fzf_in.as_bytes())
            .expect("Failed to write fzf_input to fzf command stdin");
        let output = child
            .wait_with_output()
            .expect("Failed to read fzf command stdout");
        String::from(str::from_utf8(&output.stdout).unwrap().trim())
    }
}
