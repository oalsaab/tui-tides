use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use serde::Deserialize;

use crate::app::{App, SearchMode};

use super::StyledBorder;

#[derive(Deserialize, Clone)]
pub struct TideGaugeStation {
    pub label: String,
    #[serde(rename = "stationReference")]
    pub station_reference: String,
}

#[derive(Deserialize)]
struct TideGaugeStations {
    items: Vec<TideGaugeStation>,
}

pub struct Search {
    pub input: String,
    pub mode: SearchMode,
    first_search: bool,
    exists: Option<bool>,
    pub station: Option<TideGaugeStation>,
    stations: Option<TideGaugeStations>,
}

impl Search {
    pub fn new() -> Search {
        Search {
            input: String::new(),
            mode: SearchMode::Editing,
            first_search: true,
            exists: None,
            station: None,
            stations: None,
        }
    }

    pub fn add_char(&mut self, ch: char) {
        self.input.push(ch);
    }

    pub fn remove_char(&mut self) {
        self.input.pop();
    }

    fn list_stations() -> TideGaugeStations {
        let body = reqwest::blocking::get(
            "https://environment.data.gov.uk/flood-monitoring/id/stations?type=TideGauge",
        )
        .unwrap()
        .text()
        .unwrap();

        let stations: TideGaugeStations = serde_json::from_str(&body).unwrap();
        stations
    }

    fn find_station(&mut self) {
        let input = self.input.clone().to_lowercase();

        if self.first_search {
            self.stations = Some(Search::list_stations());
            self.first_search = false
        };

        self.station = self
            .stations
            .as_ref()
            .unwrap()
            .items
            .iter()
            .find(|station| station.label.to_lowercase() == input)
            .cloned();
    }

    pub fn transition(&mut self, app: &mut App) {
        self.find_station();

        if self.station.is_some() {
            self.exists = Some(true);
            app.transition();
        } else {
            self.exists = Some(false);
        };
    }

    fn non_existent(&self) -> Block {
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .fg(Color::Red)
            .title_style(Color::White)
            .title_bottom(Line::from("Station does not exist!").centered())
    }
}

impl Widget for &mut Search {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = match self.mode {
            SearchMode::Editing => match self.exists {
                None | Some(true) => self.focused(),
                Some(false) => self.non_existent(),
            },

            SearchMode::Normal => self.default(),
        }
        .title("Search");

        let input = self.input.clone();
        let key_text = Paragraph::new(input).block(block);

        key_text.render(area, buf);
    }
}
