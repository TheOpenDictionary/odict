use crate::{Definition, Entry, Error};

use super::{example::write_example, note::write_note, utils::indent};

pub fn write_definition(
    lines: &mut Vec<String>,
    index: usize,
    level: usize,
    definition: &Definition,
    entry: &Entry,
) -> Result<(), Error> {
    let numbering = (index + 1).to_string();
    let text = &format!("{}. {}", numbering, &definition.value);

    lines.push(indent(text, level));

    for example in (&definition).examples.iter() {
        write_example(lines, level + 1, example, entry)?;
    }

    if definition.notes.len() > 0 {
        lines.push(indent("\n**Notes**\n\n", level + 1));

        for (ndx, note) in (&definition).notes.iter().enumerate() {
            write_note(lines, level + 1, ndx, note, entry)?;
        }

        lines.push("".into());
    }

    Ok(())
}
