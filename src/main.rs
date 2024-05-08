use dotenv::dotenv;
mod city;
mod helpers;
use city::City;
mod openweather;
mod weather;
use clap::Parser;

/// A simple program to get the current weather of a city.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value = "Wien")]
    query: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args = Args::parse();
    let cities: Vec<City> = openweather::search_for_city(&args.query).await?;

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
