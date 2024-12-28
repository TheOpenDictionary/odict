use crate::{Entry, Example, MarkdownStrategy};

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
        &underline_target(
            &example.value.parse(MarkdownStrategy::Disabled),
            &entry.term
        )
    );

    lines.push(indent(&text, indent_width).into());

    Ok(())
}
