use crate::{Entry, Group};

use super::definition::write_definition;

pub fn write_group(
    lines: &mut Vec<String>,
    index: usize,
    group: &Group,
    entry: &Entry,
) -> crate::Result<()> {
    let text = &format!("{}. {}", index + 1, group.description);

    lines.push(text.to_string());

    for (idx, definition) in group.definitions.iter().enumerate() {
        write_definition(lines, idx, 1, definition, entry)?;
    }

    Ok(())
}
