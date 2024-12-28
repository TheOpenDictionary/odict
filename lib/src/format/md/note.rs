use crate::{error::Result, Entry, MarkdownStrategy, Note};

use super::{
    example::write_example,
    utils::{indent, index_to_alpha},
};

pub fn write_note(lines: &mut Vec<String>, index: usize, note: &Note, entry: &Entry) -> Result<()> {
    lines.push(
        indent(
            &format!(
                "{}. {}",
                index_to_alpha(index),
                note.value.parse(MarkdownStrategy::Disabled)
            ),
            6,
        )
        .into(),
    );

    if note.examples.len() > 0 {
        lines.push("".into());

        for example in (&note).examples.iter() {
            write_example(lines, 9, example, entry)?;
        }
    }

    Ok(())
}
