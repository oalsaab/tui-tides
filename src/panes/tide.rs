use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Chart, Widget},
};
use serde::Deserialize;

use super::{ChartPane, StyledBorder};
use crate::apis::station;
use crate::app::Focused;
use crate::panes::ChartRanges;

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

        hours + (minutes / 100.0)
    }

    fn measurement_range(&self) -> ChartRanges {
        let data: Vec<f64> = self.items.iter().map(|f| f.value).collect();
        ChartRanges::build(&data)
    }

    fn dt_range(&self) -> ChartRanges {
        let data: Vec<f64> = self
            .items
            .iter()
            .map(|f| self.convert_dt(&f.date_time))
            .collect();

        ChartRanges::build(&data)
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
        if self.rendered {
            return;
        }

        let station_reference = self
            .station_reference
            .as_ref()
            .expect("Only reached when station is found");

        let body = station::Readings::new().call(&station_reference);

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

        let chart = self.create(&data, &x, &y, "Time", "mOAD");

        Chart::new(chart.datasets)
            .x_axis(chart.x_axis)
            .y_axis(chart.y_axis)
            .block(block)
            .render(area, buf);
    }
}
