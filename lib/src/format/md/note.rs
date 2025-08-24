use crate::{
    error::Result,
    schema::{Entry, Note},
};

use super::{example::write_example, utils::indent};

pub fn write_note(lines: &mut Vec<String>, index: usize, note: &Note, entry: &Entry) -> Result<()> {
    lines.push(indent(&format!("{}. {}", index + 1, note.value), 6).into());

    if note.examples.len() > 0 {
        lines.push("".into());

        for example in (&note).examples.iter() {
            write_example(lines, 9, example, entry)?;
        }
    }

    Ok(())
}
