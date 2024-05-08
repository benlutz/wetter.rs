use reqwest::Url;
use std::env;

use crate::{city::City, helpers::get_language, weather::WeatherInfo};

pub async fn get_open_weather_map_data(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    match env::var("OPEN_WEATHER_API_KEY") {
        Ok(api_key) => {
            let params = [
                ("appid", &api_key),
                ("lang", &get_language()),
                ("units", &String::from("metric")),
            ];
            let url = Url::parse_with_params(url, &params)?;

            let response: reqwest::Response = reqwest::get(url).await?;
            let body = response.text().await?;

            Ok(body)
        }
        Err(_) => Err("OPEN_WEATHER_API_KEY is not defined".into()),
    }
}

pub async fn search_for_city(query: &str) -> Result<Vec<City>, Box<dyn std::error::Error>> {
    let api_url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=5",
        query
    );

    let response = get_open_weather_map_data(&api_url).await?;

    let cities: Vec<City> = serde_json::from_str(&response)?;
    Ok(cities)
}

pub async fn get_current_weather(city: &City) -> Result<WeatherInfo, Box<dyn std::error::Error>> {
    let api_url = format!(
        "http://api.openweathermap.org/data/2.5/weather?lat={}&lon={}",
        city.lat, city.lon
    );

    let response = get_open_weather_map_data(&api_url).await?;
    let weather_info: WeatherInfo = serde_json::from_str(&response)?;
    Ok(weather_info)
}
