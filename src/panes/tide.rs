use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    symbols,
    widgets::{Axis, Chart, Dataset, GraphType, Widget},
};
use serde::Deserialize;

use crate::app::Focused;

use super::StyledBorder;

struct StationRanges {
    labels: Vec<String>,
    min: f64,
    max: f64,
}

impl StationRanges {
    fn build(data: &[f64]) -> StationRanges {
        let max = data.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
        let min = data.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
        let mid = (max + min) / 2.0;

        let labels = vec![
            format!("{:.2}", min),
            format!("{:.2}", mid),
            format!("{:.2}", max),
        ];

        StationRanges {
            labels,
            min: *min,
            max: *max,
        }
    }
}

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

    fn measurement_range(&self) -> StationRanges {
        let data: Vec<f64> = self.items.iter().map(|f| f.value).collect();
        StationRanges::build(&data)
    }

    fn dt_range(&self) -> StationRanges {
        let data: Vec<f64> = self
            .items
            .iter()
            .map(|f| self.convert_dt(&f.date_time))
            .collect();

        StationRanges::build(&data)
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
    pub readings: Option<StationReadings>,
    pub rendered: bool,
}

impl Tide {
    pub fn get_station_readings(&mut self) {
        // If we've rendered it, don't make another call to API
        if self.rendered {
            return;
        }

        let station_reference = self
            .station_reference
            .as_ref()
            .expect("Only reached when station is found");

        let url = format!("https://environment.data.gov.uk/flood-monitoring/id/stations/{}/readings?&today&_limit=100", station_reference);

        let body = reqwest::blocking::get(&url).unwrap().text().unwrap();

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
        let y = readings.measurement_range();
        let x = readings.dt_range();

        let datasets = vec![Dataset::default()
            .marker(symbols::Marker::Dot)
            .graph_type(GraphType::Scatter)
            .style(Style::default().magenta())
            .data(&data)];

        let x_axis = Axis::default()
            .title("Time".red())
            .style(Style::default().white())
            .bounds([x.min, x.max])
            .labels(x.labels.iter().map(|f| f.into()).collect());

        let y_axis = Axis::default()
            .title("mOAD".red())
            .style(Style::default().white())
            .bounds([y.min, y.max])
            .labels(y.labels.iter().map(|f| f.into()).collect());

        Chart::new(datasets)
            .x_axis(x_axis)
            .y_axis(y_axis)
            .block(block)
            .render(area, buf);
    }
}
