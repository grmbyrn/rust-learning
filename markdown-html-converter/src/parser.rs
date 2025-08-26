pub enum MarkdownElement {
    Heading { level: usize, text: String },
    Paragraph(String),
    UnorderedList(Vec<String>),
    OrderedList(Vec<String>), // Add this line
}

pub struct MarkdownDocument {
    pub elements: Vec<MarkdownElement>,
}

impl MarkdownDocument {
    pub fn from_str(input: &str) -> Self {
        let mut elements = Vec::new();
        let mut lines = input.lines().peekable();

        while let Some(line) = lines.next() {
            let trimmed = line.trim();

            // Headings
            if let Some(_stripped) = trimmed.strip_prefix('#') {
                let level = trimmed.chars().take_while(|&c| c == '#').count();
                let text = trimmed[level..].trim().to_string();
                elements.push(MarkdownElement::Heading { level, text });
            }
            // Unordered list
            else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                let mut items = Vec::new();
                let mut current = Some(trimmed);

                while let Some(item_line) = current {
                    if item_line.starts_with("- ") || item_line.starts_with("* ") {
                        items.push(item_line[2..].trim().to_string());
                        current = lines.peek().map(|l| l.trim());
                        if current.is_some() && (current.unwrap().starts_with("- ") || current.unwrap().starts_with("* ")) {
                            lines.next();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                elements.push(MarkdownElement::UnorderedList(items));
            }
            // Ordered list
            else if trimmed.chars().next().map_or(false, |c| c.is_digit(10)) && trimmed.contains(". ") {
                let mut items = Vec::new();
                let mut current = Some(trimmed);

                while let Some(item_line) = current {
                    // Check for pattern: number(s) + ". "
                    let mut parts = item_line.splitn(2, ". ");
                    if let (Some(num), Some(text)) = (parts.next(), parts.next()) {
                        if num.chars().all(|c| c.is_digit(10)) {
                            items.push(text.trim().to_string());
                            current = lines.peek().map(|l| l.trim());
                            if current.is_some()
                                && current.unwrap().chars().next().map_or(false, |c| c.is_digit(10))
                                && current.unwrap().contains(". ")
                            {
                                lines.next();
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                elements.push(MarkdownElement::OrderedList(items));
            }
            // Paragraph
            else if !trimmed.is_empty() {
                elements.push(MarkdownElement::Paragraph(trimmed.to_string()));
            }
        }

        MarkdownDocument { elements }
    }
}