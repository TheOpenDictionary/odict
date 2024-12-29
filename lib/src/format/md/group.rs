use crate::{Entry, Group};

use super::{definition::write_definition, utils::indent};

pub fn write_group(
    lines: &mut Vec<String>,
    index: usize,
    group: &Group,
    entry: &Entry,
) -> crate::Result<()> {
    let text = &format!("{}. {}", index + 1, group.description);

    lines.push(indent(text, 2));

    for (idx, definition) in group.definitions.iter().enumerate() {
        write_definition(lines, idx, 5, definition, entry)?;
    }

    Ok(())
}
