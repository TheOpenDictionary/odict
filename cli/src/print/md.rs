use console::style;
use pulldown_cmark::{html, Event, Parser, Tag, TagEnd};

fn cli_start_tag(tag: &Tag, buffer: &mut String, tags_stack: &mut Vec<Tag>) {
    match tag {
        Tag::Emphasis => style(val).bold() buffer.push_str("_"),
        Tag::Link { title, .. } | Tag::Image { title, .. } => buffer.push_str(&title),
        Tag::Item => {
            buffer.push('\n');
            let mut lists_stack = tags_stack
                .iter_mut()
                .filter_map(|tag| match tag {
                    Tag::List(nb) => Some(nb),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let prefix_tabs_count = lists_stack.len() - 1;
            for _ in 0..prefix_tabs_count {
                buffer.push('\t')
            }
            if let Some(Some(nb)) = lists_stack.last_mut() {
                buffer.push_str(&nb.to_string());
                buffer.push_str(". ");
                *nb += 1;
            } else {
                buffer.push_str("• ");
            }
        }
        Tag::Paragraph | Tag::CodeBlock(_) | Tag::Heading { .. } => buffer.push('\n'),
        _ => (),
    }
}

fn cli_end_tag(tag: &TagEnd, buffer: &mut String, tags_stack: &[Tag]) {
    match tag {
        TagEnd::Paragraph | TagEnd::Heading { .. } => buffer.push('\n'),
        TagEnd::CodeBlock { .. } => {
            if buffer.chars().last() != Some('\n') {
                buffer.push('\n');
            }
        }
        TagEnd::List(_) => {
            let is_sublist = tags_stack.iter().any(|tag| match tag {
                Tag::List(_) => true,
                _ => false,
            });
            if !is_sublist {
                buffer.push('\n')
            }
        }
        _ => (),
    }
}

pub fn md_cli(md: &str) -> String {
    let parser = Parser::new(&md);
    let mut tags_stack = Vec::new();
    let mut buffer = String::new();

    let mut italic = false;

    // For each event we push into the buffer to produce the plain text version.
    for event in parser {
        match event {
            // The start and end events don't contain the text inside the tag.
            // That's handled by the `Event::Text` arm.
            Event::Start(tag) => {
                cli_start_tag(&tag, &mut buffer, &mut tags_stack);
                tags_stack.push(tag);
            }
            Event::End(tag) => {
                tags_stack.pop();
                cli_end_tag(&tag, &mut buffer, &tags_stack);
            }
            Event::Text(content) => buffer.push_str(&content),
            Event::Code(content) => buffer.push_str(&content),
            Event::SoftBreak => buffer.push(' '),
            _ => (),
        }
    }

    buffer.trim().to_string()
}
