mod operations;
mod cli;
mod errors;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("Error: {}", e);
    }
}
