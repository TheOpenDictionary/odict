use console::Style;
use pulldown_cmark::{Event, Parser, Tag};

pub fn print_md(md: &String) -> String {
    let parser = Parser::new(md);
    let mut tags_stack = Vec::new();
    let mut buffer = String::new();

    for event in parser {
        match event {
            Event::Start(tag) => {
                tags_stack.push(tag);
            }
            Event::End(_) => {
                tags_stack.pop();
            }
            Event::Text(content) => {
                let mut style = Style::new();

                if tags_stack.contains(&Tag::Emphasis) {
                    style = style.italic();
                }

                if tags_stack.contains(&Tag::Strong) {
                    style = style.bold();
                }

                if tags_stack.contains(&Tag::Strikethrough) {
                    style = style.strikethrough();
                }

                buffer.push_str(&style.apply_to(&content).to_string());
            }
            Event::Code(content) => buffer.push_str(&content),
            Event::SoftBreak => buffer.push(' '),
            _ => (),
        }
    }

    buffer.trim().to_string()
}
