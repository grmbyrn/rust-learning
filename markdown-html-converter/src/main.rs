mod parser;
mod renderer;
mod utils;

use parser::MarkdownDocument;
use renderer::render_html;
use std::io::{self, Read};

fn main() {
    println!("Paste your Markdown below. Press Ctrl+D (on Mac/Linux) or Ctrl+Z then Enter (on Windows) when done:\n");

    // Read all input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

    println!("\n--- HTML OUTPUT ---\n");

    let doc = MarkdownDocument::from_str(&input);
    let html = render_html(&doc);
    println!("{}", html);
}
