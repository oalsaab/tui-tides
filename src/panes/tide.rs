use chrono::{DateTime, Utc};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Styled, Stylize},
    symbols,
    widgets::{Axis, Block, Chart, Dataset, GraphType, Widget},
};
use serde::Deserialize;

use crate::app::Focused;

use super::StyledBorder;

// Build chart from below (variable station)
// https://environment.data.gov.uk/flood-monitoring/id/stations/E72639/readings?&today&_limit=100

#[derive(Deserialize, Clone)]
pub struct StationReadings {
    items: Vec<StationReading>,
}

impl StationReadings {
    fn dataset(&self) -> Vec<(f64, f64)> {
        self.items
            .iter()
            .map(|x| (self.convert_dt(&x.date_time), x.value as f64))
            .collect()
    }

    fn convert_dt(&self, dt: &str) -> f64 {
        let tpart = &dt[11..16];
        let parts: Vec<&str> = tpart.split(':').collect();

        let hours: f64 = parts[0].parse().unwrap();
        let minutes: f64 = parts[1].parse().unwrap();

        hours + (minutes / 60.0)
    }

    fn measurement_range(&self) -> Vec<String> {
        let measurements: Vec<f64> = self.items.iter().map(|f| f.value).collect();

        let max = measurements.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
        let min = measurements.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
        let mid = (max + min) / 2.0;

        vec![min.to_string(), mid.to_string(), max.to_string()]
    }
}
#[derive(Deserialize, Clone)]
struct StationReading {
    #[serde(rename = "dateTime")]
    date_time: String,
    value: f64,
}

#[derive(Clone)]
pub struct Tide {
    pub focus: Focused,
    pub station_reference: Option<String>,
    pub rendered: bool,
    pub readings: Option<StationReadings>,
}

impl Tide {
    pub fn get_station_readings(&mut self) {
        if self.rendered {
            return;
        }

        let station_reference = self
            .station_reference
            .as_ref()
            .expect("Only reached when station is found");

        let url = format!("https://environment.data.gov.uk/flood-monitoring/id/stations/{}/readings?&today&_limit=100", station_reference);

        let body = reqwest::blocking::get(url).unwrap().text().unwrap();

        let readings: StationReadings = serde_json::from_str(&body).unwrap();

        self.readings = Some(readings);
        self.rendered = true;
    }
}

impl Widget for &mut Tide {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = match self.focus {
            Focused::Tide => self.focused(),
            _ => self.default(),
        }
        .title("Tide");

        let readings = self.readings.as_ref().unwrap();
        let data = readings.dataset();
        let m_labels = readings.measurement_range();

        // let data = &self.readings.as_ref().unwrap().dataset();

        let datasets = vec![Dataset::default()
            .marker(symbols::Marker::Dot)
            .graph_type(GraphType::Scatter)
            .style(Style::default().magenta())
            .data(&data)];

        // Create the X axis and define its properties
        let x_axis = Axis::default()
            .title("Time".red())
            .style(Style::default().white())
            .bounds([0.0, 24.0])
            .labels(vec!["0.0".into(), "12.0".into(), "23.45".into()]);

        // Create the Y axis and define its properties
        let y_axis = Axis::default()
            .title("mOAD".red())
            .style(Style::default().white())
            .bounds([-10.0, 25.0])
            .labels(m_labels.iter().map(|f| f.into()).collect());

        Chart::new(datasets)
            .x_axis(x_axis)
            .y_axis(y_axis)
            .block(block)
            .render(area, buf);
    }
}
