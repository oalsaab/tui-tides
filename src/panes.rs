use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    widgets::{Axis, Block, BorderType, Borders, Dataset, GraphType},
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

pub struct ChartData<'a> {
    pub datasets: Vec<Dataset<'a>>,
    pub x_axis: Axis<'a>,
    pub y_axis: Axis<'a>,
}

trait ChartPane {
    fn create<'a>(
        &'a self,
        data: &'a [(f64, f64)],
        x: &'a ChartRanges,
        y: &'a ChartRanges,
        x_title: &'a str,
        y_title: &'a str,
    ) -> ChartData {
        let datasets = vec![Dataset::default()
            .marker(symbols::Marker::Dot)
            .graph_type(GraphType::Scatter)
            .style(Style::default().magenta())
            .data(&data)];

        let x_axis = Axis::default()
            .title(x_title.red())
            .style(Style::default().white())
            .bounds([x.min, x.max])
            .labels(x.labels.iter().map(|f| f.into()).collect());

        let y_axis = Axis::default()
            .title(y_title.red())
            .style(Style::default().white())
            .bounds([y.min, y.max])
            .labels(y.labels.iter().map(|f| f.into()).collect());

        ChartData {
            datasets,
            x_axis,
            y_axis,
        }
    }
}

impl ChartPane for Tide {}
impl ChartPane for Weather {}
