use super::clear_term;
use super::fzf_selector::FzfSelector;
use std::slice::Iter;
use super::fzf_selector::SelectType;

#[derive(PartialEq, Eq, Clone)]
pub enum MenuOptions {
    MainMenu,
    MVSelector,
    ToggleMVs,
    ToggleLive,
    SortAsc,
    SortDesc,
    SortMtime,
    Random,
    Quit,
    Update,
    SearchFilter,
    ClearPlayed,
}

impl std::fmt::Display for MenuOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuOptions::MainMenu => write!(f, "Main Menu"),
            MenuOptions::MVSelector => write!(f, "MV Selector"),
            MenuOptions::ToggleMVs => write!(f, "Toggle MVs"),
            MenuOptions::ToggleLive => write!(f, "Toggle Live"),
            MenuOptions::SortAsc => write!(f, "Sort Ascending"),
            MenuOptions::SortDesc => write!(f, "Sort Descending"),
            MenuOptions::SortMtime => write!(f, "Sort by Mtime"),
            MenuOptions::Random => write!(f, "Random"),
            MenuOptions::Quit => write!(f, "Quit"),
            MenuOptions::Update => write!(f, "Update"),
            MenuOptions::SearchFilter => write!(f, "Search Filter"),
            MenuOptions::ClearPlayed => write!(f, "Clear Played"),
        }
    }
}

impl MenuOptions {
    fn iterator() -> Iter<'static, MenuOptions> {
        static OPTIONS: [MenuOptions; 12] = [
            MenuOptions::MainMenu,
            MenuOptions::MVSelector,
            MenuOptions::ToggleMVs,
            MenuOptions::ToggleLive,
            MenuOptions::SortAsc,
            MenuOptions::SortDesc,
            MenuOptions::SortMtime,
            MenuOptions::Random,
            MenuOptions::Quit,
            MenuOptions::Update,
            MenuOptions::SearchFilter,
            MenuOptions::ClearPlayed,
        ];
        OPTIONS.iter()
    }

    pub fn get_selection(selected: &str) -> Option<&MenuOptions> {
        MenuOptions::iterator().find(|&view| selected == format!("[[{}]]", view))
    }

    pub fn generate_menu(exclude: Vec<String>) -> Vec<String> {
        MenuOptions::iterator()
            .filter(|view| !exclude.contains(&view.to_string()))
            .map(|view| format!("[[{}]]", view))
            .collect()
    }
}

pub struct MainMenu {
    view_type: MenuOptions,
    header: String,
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            view_type: MenuOptions::MainMenu,
            header: "Main Menu".to_owned(),
        }
    }
}

impl MainMenu {
    pub fn start(&mut self) -> MenuOptions {
        loop {
            clear_term(&self.header)
                .unwrap_or_else(|e| eprintln!("Couldn't clear terminal: {}", e));
            let menu = MenuOptions::generate_menu(vec![self.view_type.to_string()]);
            let fzf_view = FzfSelector::new(None, Some(menu.clone()), None);
            let selected = fzf_view.fzf_select(SelectType::Single);
            if let Some(view) = MenuOptions::get_selection(&selected) {
                return view.clone();
            }
        }
    }
}
