use crate::{openweather, weather::WeatherInfo};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct City {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

impl City {
    pub async fn get_current_weather(&self) -> Result<WeatherInfo, Box<dyn std::error::Error>> {
        openweather::get_current_weather(self).await
    }
}
