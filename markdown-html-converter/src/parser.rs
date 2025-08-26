pub enum MarkdownElement {
    Heading { level: usize, text: String },
    Paragraph(String),
}

pub struct MarkdownDocument {
    pub elements: Vec<MarkdownElement>,
}

impl MarkdownDocument {
    pub fn from_str(input: &str) -> Self {
        let mut elements = Vec::new();
        for line in input.lines() {
            let trimmed = line.trim();
            if let Some(_stripped) = trimmed.strip_prefix('#') {
                // Count heading level
                let level = trimmed.chars().take_while(|&c| c == '#').count();
                let text = trimmed[level..].trim().to_string();
                elements.push(MarkdownElement::Heading { level, text });
            } else if !trimmed.is_empty() {
                elements.push(MarkdownElement::Paragraph(trimmed.to_string()));
            }
        }
        MarkdownDocument { elements }
    }
}