use crate::apis::meteo;
use crate::app::Focused;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    symbols,
    widgets::{Axis, Chart, Dataset, GraphType, Widget},
};

use serde::Deserialize;

use super::StyledBorder;
use crate::panes::ChartRanges;

#[derive(Deserialize, Clone)]
pub struct TemperatureReadings {
    minutely_15: Minutely,
}

#[derive(Deserialize, Clone)]
struct Minutely {
    time: Vec<String>,
    #[serde(rename = "temperature_2m")]
    temperature: Vec<f64>,
}

impl Minutely {
    fn dataset(&self) -> Vec<(f64, f64)> {
        let converted_time: Vec<f64> = self.time.iter().map(|f| self.convert_dt(f)).collect();

        std::iter::zip(converted_time.clone(), self.temperature.clone()).collect()
    }

    fn convert_dt(&self, dt: &str) -> f64 {
        let parts = dt.split_once('T').unwrap().1.split_once(':').unwrap();

        let hours: f64 = parts.0.parse().unwrap();
        let minutes: f64 = parts.1.parse().unwrap();

        hours + (minutes / 100.0)
    }

    fn temp_range(&self) -> ChartRanges {
        ChartRanges::build(&self.temperature)
    }

    fn dt_range(&self) -> ChartRanges {
        ChartRanges::build(
            &self
                .time
                .iter()
                .map(|f| self.convert_dt(f))
                .collect::<Vec<f64>>(),
        )
    }
}

#[derive(Clone)]
pub struct Weather {
    pub focus: Focused,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub readings: Option<TemperatureReadings>,
    pub rendered: bool,
}

impl Weather {
    pub fn get_temperature_readings(&mut self) {
        if self.rendered {
            return;
        }

        let body = meteo::Meteo::new().call(self.lat.unwrap(), self.lon.unwrap());

        let readings: TemperatureReadings = serde_json::from_str(&body).unwrap();

        self.readings = Some(readings);
        self.rendered = true;
    }
}

impl Widget for &mut Weather {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = match self.focus {
            Focused::Weather => self.focused(),
            _ => self.default(),
        }
        .title("Weather");

        let readings = &self.readings.as_ref().unwrap().minutely_15;

        let data = readings.dataset();
        let y = readings.temp_range();
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
            .title("Temperature".red())
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
