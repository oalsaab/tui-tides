const ROOT: &str = "https://environment.data.gov.uk";
const PATH: &str = "flood-monitoring/id/stations";

pub enum Period {
    Today,
    Latest,
}

pub struct Stations {
    pub unitname: String,
}

impl Stations {
    pub fn new() -> Stations {
        Stations {
            unitname: String::from("mAOD"),
        }
    }

    pub fn call(&self) -> String {
        let api = format!("{}/{}?", ROOT, PATH);
        let params = [("type", "TideGauge"), ("unitName", &self.unitname)];

        let url = reqwest::Url::parse_with_params(&api, &params).unwrap();

        reqwest::blocking::get(url).unwrap().text().unwrap()
    }
}

pub struct Readings {
    pub duration: Period,
    pub limit: i8,
}

impl Readings {
    pub fn new() -> Readings {
        Readings {
            duration: Period::Today,
            limit: 100,
        }
    }

    pub fn call(&self, station: &str) -> String {
        let dur = match self.duration {
            Period::Today => "today",
            Period::Latest => "latest",
        };

        let api = format!("{}/{}/{}/readings?&{}", ROOT, PATH, station, dur);

        let params = [("_limit", &self.limit.to_string())];

        let url = reqwest::Url::parse_with_params(&api, &params).unwrap();

        reqwest::blocking::get(url).unwrap().text().unwrap()
    }
}
