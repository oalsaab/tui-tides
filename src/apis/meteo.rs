const ROOT: &str = "https://api.open-meteo.com";
const PATH: &str = "v1/forecast";

pub struct Meteo {
    pub temperature: String,
    pub start_date: String,
    pub end_date: String,
}

impl Meteo {
    pub fn new() -> Meteo {
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

        Meteo {
            temperature: String::from("temperature_2m"),
            start_date: today.clone(),
            end_date: today.clone(),
        }
    }

    pub fn call(&self, lat: f64, lon: f64) -> String {
        let api = format!("{}/{}?", ROOT, PATH);

        let params = [
            ("latitude", &lat.to_string()),
            ("longitude", &lon.to_string()),
            ("minutely_15", &self.temperature),
            ("start_date", &self.start_date),
            ("end_date", &self.end_date),
        ];

        let url = reqwest::Url::parse_with_params(&api, &params).unwrap();

        reqwest::blocking::get(url).unwrap().text().unwrap()
    }
}
