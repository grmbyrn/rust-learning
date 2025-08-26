mod parser;
mod renderer;
mod utils;

use parser::MarkdownDocument;
use renderer::render_html;
use std::io::{self, Read};

fn main() {
    // Read all input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
    let doc = MarkdownDocument::from_str(&input);
    let html = render_html(&doc);
    println!("{}", html);
}
