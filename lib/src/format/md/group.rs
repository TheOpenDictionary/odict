use crate::{Entry, Group, MarkdownStrategy};

use super::{definition::write_definition, utils::indent};

pub fn write_group(
    lines: &mut Vec<String>,
    index: usize,
    group: &Group,
    entry: &Entry,
) -> crate::Result<()> {
    let text = &format!(
        "{}. {}",
        index + 1,
        group.description.parse(MarkdownStrategy::Disabled)
    );

    lines.push(indent(text, 2));

    for (idx, definition) in group.definitions.iter().enumerate() {
        write_definition(lines, idx, 5, true, definition, entry)?;
    }

    Ok(())
}
