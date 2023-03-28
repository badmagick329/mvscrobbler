pub mod updater;
use super::avmod::AudioVideoData;
use crossterm::{cursor, terminal, QueueableCommand};
use std::io::{stdout, Write};
use std::process::{Command, Stdio};
use std::slice::Iter;
use std::str;

pub struct FzfSelector {
    inputs: Vec<String>,
    other_options: Vec<String>,
    height: String,
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

    pub fn fzf_select(self) -> String {
        let mut child = Command::new("fzf")
            .args(["--height", self.height.as_str(), "--reverse"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        let mut fzf_in = String::new();
        for input in self.inputs.iter() {
            fzf_in.push_str(input);
            fzf_in.push('\n');
        }
        for option in self.other_options.iter() {
            fzf_in.push_str(option);
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

fn clear_term(header: &str) -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?;
    write!(stdout, "{}", header)?;
    stdout.flush()
}

#[derive(PartialEq, Eq, Clone)]
pub enum ViewTypes {
    MainMenu,
    MVSelector,
    ToggleMVs,
    ToggleLive,
    Quit,
}

impl std::fmt::Display for ViewTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViewTypes::MainMenu => write!(f, "Main Menu"),
            ViewTypes::MVSelector => write!(f, "MV Selector"),
            ViewTypes::ToggleMVs => write!(f, "Toggle MVs"),
            ViewTypes::ToggleLive => write!(f, "Toggle Live"),
            ViewTypes::Quit => write!(f, "Quit"),
        }
    }
}

impl ViewTypes {
    fn iterator() -> Iter<'static, ViewTypes> {
        static VIEWS: [ViewTypes; 5] = [
            ViewTypes::MainMenu,
            ViewTypes::MVSelector,
            ViewTypes::ToggleMVs,
            ViewTypes::ToggleLive,
            ViewTypes::Quit,
        ];
        VIEWS.iter()
    }

    pub fn other_views(&self) -> Vec<String> {
        ViewTypes::iterator()
            .filter(|view| *view != self)
            .map(|view| view.to_string())
            .collect()
    }
    pub fn get_selection(selected: &str) -> Option<&ViewTypes> {
        ViewTypes::iterator().find(|&view| selected == format!("[[{}]]", view))
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum FilterTypes {
    MVs,
    Live,
}

impl std::fmt::Display for FilterTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterTypes::MVs => write!(f, "MVs"),
            FilterTypes::Live => write!(f, "Live"),
        }
    }
}

pub struct MVSelector {
    view_type: ViewTypes,
    avd: AudioVideoData,
    header: String,
    filters: Vec<FilterTypes>,
}

trait ViewsMenu {
    fn generate_menu(&self, view_type: &ViewTypes) -> Vec<String> {
        let mut menu = Vec::new();
        for view in view_type.other_views().iter() {
            menu.push(format!("[[{}]]", view));
        }
        menu
    }
}

impl ViewsMenu for MVSelector {}

/// UI Entrypoint
impl MVSelector {
    pub fn new(avd: AudioVideoData) -> Self {
        Self {
            view_type: ViewTypes::MVSelector,
            avd,
            header: "Search for an MV or search quit to exit".to_owned(),
            filters: Vec::new(),
        }
    }

    pub async fn start(&mut self) -> ViewTypes {
        loop {
            clear_term(&self.header)
                .unwrap_or_else(|e| eprintln!("Couldn't clear terminal: {}", e));
            let filtered_list = self.filtered_list();
            let menu = self.generate_menu(&self.view_type);
            let fzf_view = FzfSelector::new(Some(self.filtered_list()), Some(menu.clone()), None);
            let selected = fzf_view.fzf_select();
            if let Some(view) = ViewTypes::get_selection(&selected) {
                return view.clone();
            }
            self.avd.play_media(&selected).await;
            self.header = format!(
                "Playing {}\n\nSearch for an MV or search quit to exit",
                selected
            );
        }
    }
    fn filtered_list(&mut self) -> Vec<String> {
        self.avd
            .list_videos()
            .iter()
            .filter(|video| {
                let mv_name = video.split(" - ").last();
                if mv_name.is_none() {
                    return false;
                }
                let is_live = mv_name.unwrap().contains('(') && mv_name.unwrap().contains(')');
                if self.filters.contains(&FilterTypes::MVs) && !is_live {
                    return false;
                }
                if self.filters.contains(&FilterTypes::Live) && is_live {
                    return false;
                }
                true
            })
            .map(|video| video.to_owned())
            .collect()
    }

    fn filter_video(&self, video: &str) -> bool {
        let mv_name = video.split(" - ").last();
        if mv_name.is_none() {
            return false;
        }
        let is_live = mv_name.unwrap().contains('(') && mv_name.unwrap().contains(')');
        if self.filters.contains(&FilterTypes::MVs) && !is_live {
            return false;
        }
        if self.filters.contains(&FilterTypes::Live) && is_live {
            return false;
        }
        true
    }

    pub fn toggle_filter(&mut self, filter: FilterTypes) -> ViewTypes {
        if self.filters.contains(&filter) {
            self.filters.retain(|f| *f != filter);
        } else {
            self.filters.push(filter);
        }
        ViewTypes::MVSelector
    }
}

pub struct MainMenu {
    view_type: ViewTypes,
    header: String,
}

impl ViewsMenu for MainMenu {}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            view_type: ViewTypes::MainMenu,
            header: "Main Menu".to_owned(),
        }
    }
}

impl MainMenu {
    pub fn start(&mut self) -> ViewTypes {
        loop {
            clear_term(&self.header)
                .unwrap_or_else(|e| eprintln!("Couldn't clear terminal: {}", e));
            let menu = self.generate_menu(&self.view_type);
            let fzf_view = FzfSelector::new(None, Some(menu.clone()), None);
            let selected = fzf_view.fzf_select();
            if let Some(view) = ViewTypes::get_selection(&selected) {
                return view.clone();
            }
        }
    }
}

