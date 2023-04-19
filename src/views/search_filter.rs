use crate::views::clear_term;

use super::fzf_selector::{FzfSelector, SelectType};

pub struct SearchFilters {
    pub video_list: Vec<String>,
}

impl SearchFilters {
    pub fn new(video_list: Vec<String>) -> Self {
        Self { video_list }
    }

    pub fn start(&mut self) -> Vec<String> {
        clear_term("Multi Select files to add to search filters").unwrap_or_else(|e| eprintln!("Couldn't clear terminal: {}", e));
        let fzf_view = FzfSelector::new(
            Some(self.video_list.clone()),
            Some(vec!["[[Back]]".to_owned()]),
            None,
        );
        let selections = fzf_view.fzf_select(SelectType::Multi);
        selections
            .split('\n')
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()
    }
}
