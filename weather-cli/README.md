# Weather CLI

A command-line tool to fetch and display weather information for a given city.

## Features

- Fetch current weather by city name
- Output in a user-friendly format
- Graceful error handling (network, parsing, invalid input)
- API key management via environment variable or config file
- Help and version flags

## Usage

```sh
weather-cli --city "London"
```

## Setup

1. Get a free API key from [OpenWeatherMap](https://openweathermap.org/api).
2. Set your API key in a `.env` file:
   ```
   WEATHER_API_KEY=your_api_key_here
   ```

## Dependencies

- [clap](https://crates.io/crates/clap) for CLI argument parsing
- [reqwest](https://crates.io/crates/reqwest) for HTTP requests
- [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json) for JSON parsing
- [dotenv](https://crates.io/crates/dotenv) for environment variable management

## Project Structure

- `main.rs`: CLI entry point
- `cli.rs`: Argument parsing
- `weather.rs`: Weather API logic
- `config.rs`: API key management

## Example Output

```sh
$ weather-cli --city "Barcelona" --units metric
Barcelona: clear sky (25.5°C)
```

## Troubleshooting

- **City not found:** Check the spelling or try a different city.
- **Invalid API key:** Ensure your `.env` file contains a valid `OPENWEATHER_API_KEY`.
- **Network error:** Check your internet connection.

## License

MIT
