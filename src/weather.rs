use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherInfo {
    pub weather: Vec<Weather>,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub description: String,
}
