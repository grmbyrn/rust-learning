use crate::parser::{MarkdownDocument, MarkdownElement};

pub fn render_html(doc: &MarkdownDocument) -> String {
    let mut html = String::new();
    for el in &doc.elements {
        match el {
            MarkdownElement::Heading { level, text } => {
                html.push_str(&format!("<h{lvl}>{}</h{lvl}>\n", text, lvl = level));
            }
            MarkdownElement::Paragraph(text) => {
                html.push_str(&format!("<p>{}</p>\n", text));
            }
        }
    }
    html
}