use serde::Deserialize;
use reqwest::StatusCode;
use thiserror::Error;
use crate::cli::Units;

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub name: String,
    pub weather: Vec<Weather>,
    pub main: Main,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f64,
}

#[derive(Error, Debug)]
pub enum WeatherError {
    #[error("City not found")]
    CityNotFound,
    #[error("Invalid API key")]
    InvalidApiKey,
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Unexpected error: {0}")]
    Other(String),
}

pub async fn fetch_weather(
    city: &str,
    api_key: &str,
    units: Units,
) -> Result<WeatherResponse, WeatherError> {
    let units_str = match units {
        Units::Metric => "metric",
        Units::Imperial => "imperial",
        Units::Standard => "standard",
    };
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units={}",
        city, api_key, units_str
    );
    let resp = reqwest::get(&url).await?;

    match resp.status() {
        StatusCode::OK => {
            let weather = resp.json::<WeatherResponse>().await?;
            Ok(weather)
        }
        StatusCode::NOT_FOUND => Err(WeatherError::CityNotFound),
        StatusCode::UNAUTHORIZED => Err(WeatherError::InvalidApiKey),
        code => Err(WeatherError::Other(format!("API returned status: {}", code))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_weather_response() {
        let data = r#"
        {
            "name": "Barcelona",
            "weather": [
                { "description": "clear sky" }
            ],
            "main": { "temp": 25.5 }
        }
        "#;

        let parsed: WeatherResponse = serde_json::from_str(data).unwrap();
        assert_eq!(parsed.name, "Barcelona");
        assert_eq!(parsed.weather[0].description, "clear sky");
        assert_eq!(parsed.main.temp, 25.5);
    }
}