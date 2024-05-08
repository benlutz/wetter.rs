use dotenv::dotenv;
mod city;
mod helpers;
use city::City;
mod openweather;
mod weather;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let query = "Wien";
    let cities: Vec<City> = openweather::search_for_city(query).await?;

    if let Some(local_names) = &cities[0].local_names {
        if let Some(name) = local_names.get(&helpers::get_language()) {
            println!(
                "{}: {}",
                name.as_ref().unwrap(),
                cities[0].get_current_weather().await?.weather[0].description
            );
        }
    }

    Ok(())
}
