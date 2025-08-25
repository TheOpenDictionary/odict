use crate::schema::{Entry, Example};

use super::pronunciation::write_pronunciation;
use super::utils::{indent, underline_target};

pub fn write_example(
    lines: &mut Vec<String>,
    indent_width: usize,
    example: &Example,
    entry: &Entry,
) -> crate::Result<()> {
    let text = format!(
        "{} {}",
        "- ",
        &underline_target(&example.value, &entry.term)
    );

    lines.push(indent(&text, indent_width));

    // Add pronunciations if present
    if !example.pronunciations.is_empty() {
        // Increase indent for pronunciation details
        let pronunciation_indent = indent_width + 2;
        for pronunciation in &example.pronunciations {
            let mut pronunciation_lines = Vec::new();
            write_pronunciation(&mut pronunciation_lines, pronunciation, entry)?;

            // Apply indentation to each generated pronunciation line
            for line in pronunciation_lines {
                if !line.is_empty() {
                    lines.push(indent(&line, pronunciation_indent));
                } else {
                    lines.push(String::new());
                }
            }
        }
    }

    Ok(())
}
