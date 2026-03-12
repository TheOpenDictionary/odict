use pulldown_cmark::{html, Event, Parser, Tag, TagEnd};

fn pt_start_tag(tag: &Tag, buffer: &mut String, tags_stack: &mut Vec<Tag>) {
    match tag {
        Tag::Link { title, .. } | Tag::Image { title, .. } => buffer.push_str(title),
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

fn pt_end_tag(tag: &TagEnd, buffer: &mut String, tags_stack: &[Tag]) {
    match tag {
        TagEnd::Paragraph | TagEnd::Heading { .. } => buffer.push('\n'),
        TagEnd::CodeBlock => {
            if !buffer.ends_with('\n') {
                buffer.push('\n');
            }
        }
        TagEnd::List(_) => {
            let is_sublist = tags_stack.iter().any(|tag| matches!(tag, Tag::List(_)));
            if !is_sublist {
                buffer.push('\n')
            }
        }
        _ => (),
    }
}

pub fn to_html(md: &str) -> String {
    let parser = Parser::new(md);

    let mut html_output = String::new();

    html::push_html(&mut html_output, parser);

    html_output
}

pub fn to_text(md: &str) -> String {
    // Implementation adapted from
    // https://github.com/fbecart/markdown_to_text/blob/master/src/lib.rs

    let parser = Parser::new(md);
    let mut tags_stack = Vec::new();
    let mut buffer = String::new();

    // For each event we push into the buffer to produce the plain text version.
    for event in parser {
        match event {
            // The start and end events don't contain the text inside the tag.
            // That's handled by the `Event::Text` arm.
            Event::Start(tag) => {
                pt_start_tag(&tag, &mut buffer, &mut tags_stack);
                tags_stack.push(tag);
            }
            Event::End(tag) => {
                tags_stack.pop();
                pt_end_tag(&tag, &mut buffer, &tags_stack);
            }
            Event::Text(content) => buffer.push_str(&content),
            Event::Code(content) => buffer.push_str(&content),
            Event::SoftBreak => buffer.push(' '),
            _ => (),
        }
    }

    buffer.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::{to_html, to_text};

    #[test]
    fn test_html() {
        let md = "**This** is a <sup>test</sup> <sub>of</sub> the _parser_";
        let expected =
            "<p><strong>This</strong> is a <sup>test</sup> <sub>of</sub> the <em>parser</em></p>\n";
        let actual = to_html(md);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_text() {
        let md = "**This** is a <sup>test</sup> <sub>of</sub> the _parser_";
        let expected = "This is a test of the parser";
        let actual = to_text(md);

        assert_eq!(expected, actual);
    }
}
