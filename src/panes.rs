use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub mod search;
pub mod tide;
pub mod weather;

use search::Search;
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
impl StyledBorder for Tide {}

pub struct PaneLayout {
    pub full: Rect,
    pub top: Rect,
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

        PaneLayout {
            full: layout[0],
            top: main[0],
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
    pub tide: Tide,
    pub weather: Weather,
}

impl Pane {
    pub fn build(app: &mut App) -> Pane {
        let focus = app.focus;

        Pane {
            tide: Pane::build_tide(focus),
            weather: Pane::build_weather(focus),
        }
    }

    fn build_tide(focus: Focused) -> Tide {
        Tide {
            focus,
            station_reference: None,
            readings: None,
            rendered: false,
        }
    }

    fn build_weather(focus: Focused) -> Weather {
        Weather {
            focus,
            lat: None,
            lon: None,
            readings: None,
            rendered: false,
        }
    }
}

pub struct ChartRanges {
    pub labels: Vec<String>,
    pub min: f64,
    pub max: f64,
}

impl ChartRanges {
    pub fn build(data: &[f64]) -> ChartRanges {
        let max = data.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
        let min = data.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
        let mid = (max + min) / 2.0;

        let labels = vec![
            format!("{:.2}", min),
            format!("{:.2}", mid),
            format!("{:.2}", max),
        ];

        ChartRanges {
            labels,
            min: *min,
            max: *max,
        }
    }
}
