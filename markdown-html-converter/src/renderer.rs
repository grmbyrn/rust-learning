use crate::parser::{MarkdownDocument, MarkdownElement};

fn render_inline(text: &str) -> String {
    // Bold: **text**
    let mut html = String::new();
    let mut rest = text;
    while let Some(start) = rest.find("**") {
        html.push_str(&rest[..start]);
        if let Some(end) = rest[start + 2..].find("**") {
            html.push_str("<strong>");
            html.push_str(&rest[start + 2..start + 2 + end]);
            html.push_str("</strong>");
            rest = &rest[start + 2 + end + 2..];
        } else {
            html.push_str(&rest[start..]);
            rest = "";
            break;
        }
    }
    html.push_str(rest);

    // Italic: _text_
    let mut result = String::new();
    let mut rest = html.as_str();
    while let Some(start) = rest.find('_') {
        result.push_str(&rest[..start]);
        if let Some(end) = rest[start + 1..].find('_') {
            result.push_str("<em>");
            result.push_str(&rest[start + 1..start + 1 + end]);
            result.push_str("</em>");
            rest = &rest[start + 1 + end + 1..];
        } else {
            result.push_str(&rest[start..]);
            rest = "";
            break;
        }
    }
    result.push_str(rest);
    result
}

pub fn render_html(doc: &MarkdownDocument) -> String {
    let mut html = String::new();
    for el in &doc.elements {
        match el {
            MarkdownElement::Heading { level, text } => {
                html.push_str(&format!("<h{lvl}>{}</h{lvl}>\n", render_inline(text), lvl = level));
            }
            MarkdownElement::Paragraph(text) => {
                html.push_str(&format!("<p>{}</p>\n", render_inline(text)));
            }
            MarkdownElement::UnorderedList(items) => {
                html.push_str("<ul>\n");
                for item in items {
                    html.push_str(&format!("  <li>{}</li>\n", render_inline(item)));
                }
                html.push_str("</ul>\n");
            }
            MarkdownElement::OrderedList(items) => {
                html.push_str("<ol>\n");
                for item in items {
                    html.push_str(&format!("  <li>{}</li>\n", render_inline(item)));
                }
                html.push_str("</ol>\n");
            }
        }
    }
    html
}