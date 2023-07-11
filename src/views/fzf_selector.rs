use std::io::Write;
use std::process::{Command, Stdio};
use std::str;

pub struct FzfSelector {
    inputs: Vec<String>,
    other_options: Vec<String>,
    height: String,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SelectType {
    Single,
    Multi,
}

impl FzfSelector {
    pub fn new(
        inputs: Option<Vec<String>>,
        other_options: Option<Vec<String>>,
        height: Option<String>,
    ) -> FzfSelector {
        Self {
            inputs: inputs.unwrap_or(Vec::new()),
            other_options: other_options.unwrap_or(Vec::new()),
            height: height.unwrap_or("80%".to_string()),
        }
    }

    pub fn fzf_select(self, select_type: SelectType) -> String {
        let mut fzf_in = String::new();
        for input in self.inputs.iter() {
            fzf_in.push_str(input);
            fzf_in.push('\n');
        }
        for option in self.other_options.iter() {
            fzf_in.push_str(option);
            fzf_in.push('\n');
        }
        let args = match select_type {
            SelectType::Single => vec![
                "-i",
                "--height",
                self.height.as_str(),
                "--reverse",
                "--tiebreak=begin",
            ],
            SelectType::Multi => vec![
                "-i",
                "--height",
                self.height.as_str(),
                "--reverse",
                "--tiebreak=begin",
                "-m",
            ],
        };
        let mut child = Command::new("fzf")
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin
            .write_all(fzf_in.as_bytes())
            .expect("Failed to write fzf_input to fzf command stdin");
        let output = child
            .wait_with_output()
            .expect("Failed to read fzf command stdout");
        String::from(str::from_utf8(&output.stdout).unwrap().trim())
    }
}
