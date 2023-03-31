use super::super::avmod::AudioVideoData;
use super::clear_term;
use super::fzf_selector::FzfSelector;
use super::menu::MenuOptions;
use std::slice::Iter;
use std::str;

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
    view_type: MenuOptions,
    pub avd: AudioVideoData,
    header: String,
    filters: Vec<FilterTypes>,
}

/// UI Entrypoint
impl MVSelector {
    pub fn new(avd: AudioVideoData) -> Self {
        Self {
            view_type: MenuOptions::MVSelector,
            avd,
            header: "Search for an MV or search quit to exit".to_owned(),
            filters: Vec::new(),
        }
    }

    pub async fn start(&mut self) -> MenuOptions {
        loop {
            clear_term(&self.header)
                .unwrap_or_else(|e| eprintln!("Couldn't clear terminal: {}", e));
            let filtered_list = self.filtered_list();
            let menu = MenuOptions::generate_menu(vec![self.view_type.to_string()]);
            let fzf_view = FzfSelector::new(Some(self.filtered_list()), Some(menu.clone()), None);
            let selected = fzf_view.fzf_select();
            if let Some(view) = MenuOptions::get_selection(&selected) {
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

    pub fn toggle_filter(&mut self, filter: FilterTypes) -> MenuOptions {
        if self.filters.contains(&filter) {
            self.filters.retain(|f| *f != filter);
        } else {
            self.filters.push(filter);
        }
        MenuOptions::MVSelector
    }
}
