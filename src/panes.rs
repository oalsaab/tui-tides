use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub mod search;
pub mod sunset;
pub mod tide;
pub mod weather;

use search::Search;
use sunset::Sunset;
use tide::Tide;
use weather::Weather;

use crate::app::{App, Focused};

trait StyledBorder {
    fn focused(&self) -> Block {
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .fg(Color::Cyan)
            .title_style(Color::White)
    }

    fn default(&self) -> Block {
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .fg(Color::DarkGray)
            .title_style(Color::White)
    }
}

impl StyledBorder for Search {}
impl StyledBorder for Weather {}
impl StyledBorder for Sunset {}
impl StyledBorder for Tide {}

pub struct PaneLayout {
    pub full: Rect,
    pub top_right: Rect,
    pub top_left: Rect,
    pub bottom: Rect,
}

impl PaneLayout {
    pub fn build(f: &mut Frame) -> PaneLayout {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100), Constraint::Min(3)])
            .split(f.size());

        let main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[0]);

        let top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main[0]);

        PaneLayout {
            full: layout[0],
            top_right: top[1],
            top_left: top[0],
            bottom: main[1],
        }
    }
}

pub struct SearchLayout {
    pub area: Rect,
}

impl SearchLayout {
    pub fn build(f: &mut Frame) -> SearchLayout {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100), Constraint::Min(3)])
            .split(f.size());

        let search_box = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5), Constraint::Min(0)])
            .split(layout[0]);

        SearchLayout {
            area: search_box[0],
        }
    }
}

pub struct Pane {
    pub sunset: Sunset,
    pub tide: Tide,
    pub weather: Weather,
}

impl Pane {
    pub fn build(app: &mut App, search: &mut Search) -> Pane {
        let focus = app.focus;

        Pane {
            sunset: Pane::build_sunset(focus),
            tide: Pane::build_tide(focus),
            weather: Pane::build_weather(focus),
        }
    }

    fn build_tide(focus: Focused) -> Tide {
        Tide {
            focus,
            station_reference: None,
            rendered: false,
            readings: None,
        }
    }

    fn build_sunset(focus: Focused) -> Sunset {
        Sunset { focus }
    }

    fn build_weather(focus: Focused) -> Weather {
        Weather { focus }
    }
}
