mod cli;
mod weather;

use clap::Parser;
use cli::Cli;
use dotenv::dotenv;
use std::env;
use colored::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Cli::parse();

    let api_key = env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY not set in .env");

    match weather::fetch_weather(&args.city, &api_key, args.units).await {
        Ok(resp) => {
            let desc = resp.weather.get(0)
                .map(|w| w.description.as_str())
                .unwrap_or("N/A");
            println!(
                "{}: {} ({}°{})",
                resp.name.bold().cyan(),
                desc.yellow(),
                resp.main.temp.to_string().bold().green(),
                match args.units {
                    cli::Units::Metric => "C",
                    cli::Units::Imperial => "F",
                    cli::Units::Standard => "K",
                }
            );
        }
        Err(e) => {
            match e {
                weather::WeatherError::CityNotFound => eprintln!("City not found. Please check the city name."),
                weather::WeatherError::InvalidApiKey => eprintln!("Invalid API key. Please check your .env file."),
                weather::WeatherError::Network(err) => eprintln!("Network error: {}", err),
                weather::WeatherError::Other(msg) => eprintln!("Error: {}", msg),
            }
        }
    }
}
