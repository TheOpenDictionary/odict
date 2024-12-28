use crate::{Definition, Entry, Error, MarkdownStrategy};

use super::{
    example::write_example,
    note::write_note,
    utils::{indent, index_to_alpha},
};

pub fn write_definition(
    lines: &mut Vec<String>,
    index: usize,
    indent_width: usize,
    use_alpha: bool,
    definition: &Definition,
    entry: &Entry,
) -> Result<(), Error> {
    let numbering = match use_alpha {
        false => (index + 1).to_string(),
        true => index_to_alpha(index).to_string(),
    };

    let text = &format!(
        "{}. {}",
        numbering,
        &definition.value.parse(MarkdownStrategy::Disabled)
    );

    lines.push(indent(text, indent_width));

    for example in (&definition).examples.iter() {
        write_example(lines, indent_width + 3, example, entry)?;
    }

    if definition.notes.len() > 0 {
        lines.push(indent("\n**Notes**\n\n", indent_width + 3));

        for (ndx, note) in (&definition).notes.iter().enumerate() {
            write_note(lines, ndx, note, entry)?;
        }

        lines.push("".into());
    }

    Ok(())
}
