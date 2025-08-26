use clap::{Parser, ValueEnum};

/// Command-line arguments for the Weather CLI.
#[derive(Parser, Debug)]
pub struct Cli {
    /// City name to fetch weather for
    #[arg(short, long)]
    pub city: String,

    /// Units for temperature (metric, imperial, standard)
    #[arg(short, long, value_enum, default_value = "metric")]
    pub units: Units,
}

/// Supported units for temperature.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Units {
    Metric,
    Imperial,
    Standard,
}