#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum Focused {
    #[default]
    Weather,
    Tide,
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum SearchMode {
    #[default]
    Editing,
    Normal,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum View {
    Search(SearchMode),
    Compressed,
    Enlarged,
}

impl Default for View {
    fn default() -> Self {
        View::Search(SearchMode::Editing)
    }
}

#[derive(Default, Copy, Clone)]
pub struct App {
    pub focus: Focused,
    pub state: RunningState,
    pub view: View,
}

impl App {
    pub fn cycle(&mut self) {
        self.focus = match self.focus {
            Focused::Weather => Focused::Tide,
            Focused::Tide => Focused::Weather,
        };
    }

    pub fn quit(&mut self) {
        self.state = RunningState::Done
    }

    pub fn transition(&mut self) {
        self.view = match self.view {
            View::Search(_) => View::Compressed,
            View::Compressed => View::Enlarged,
            View::Enlarged => View::Enlarged,
        };
    }

    pub fn escape(&mut self) {
        self.view = match self.view {
            View::Search(SearchMode::Editing) => View::Search(SearchMode::Normal),
            View::Search(SearchMode::Normal) => View::Search(SearchMode::Normal),
            View::Compressed => View::Search(SearchMode::Normal),
            View::Enlarged => View::Compressed,
        };
    }

    pub fn toggle_search(&mut self, search_mode: &SearchMode) {
        self.view = match search_mode {
            SearchMode::Editing => View::Search(SearchMode::Normal),
            SearchMode::Normal => View::Search(SearchMode::Editing),
        };
    }
}
