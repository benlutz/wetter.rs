use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherInfo {
    pub weather: Vec<Weather>,
    pub main: WeatherMain,
    pub wind: Wind,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
}

#[derive(Deserialize, Debug)]
pub struct WeatherMain {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i64,
    pub humidity: i64,
}

impl WeatherInfo {
    pub fn get_temperature(&self) -> String {
        format!("{}°C", self.main.temp)
    }

    pub fn print_weather_summary(&self) {
        println!("Temperature: {}", self.get_temperature());

        println!(
            "Wind: {}m/s {}",
            self.wind.speed,
            self.wind.get_wind_direction_arrow()
        );
    }
}

impl Wind {
    pub fn get_wind_direction_arrow(&self) -> char {
        let degree = self.deg;

        let directions = [
            (0..=11, '↓'),    // North
            (12..=33, '↙'),   // North-Northwest
            (34..=56, '←'),   // Northwest
            (57..=78, '↖'),   // West-Northwest
            (79..=101, '←'),  // West
            (102..=123, '↖'), // West-Southwest
            (124..=146, '↑'), // Southwest
            (147..=168, '↗'), // South-Southwest
            (169..=191, '↑'), // South
            (192..=213, '↗'), // South-Southeast
            (214..=236, '→'), // Southeast
            (237..=258, '↘'), // East-Southeast
            (259..=281, '→'), // East
            (282..=303, '↘'), // East-Northeast
            (304..=326, '↓'), // Northeast
            (327..=348, '↙'), // North-Northeast
            (349..=360, '↓'), // North
        ];

        for (range, arrow) in directions.iter() {
            if range.contains(&degree) {
                return *arrow;
            }
        }

        // Default to '↓' if the direction is not found
        '↓'
    }
}
