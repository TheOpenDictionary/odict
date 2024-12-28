use crate::{Entry, Etymology, MarkdownStrategy};

use super::sense::write_sense;

pub fn write_ety(
    lines: &mut Vec<String>,
    etymology: &Etymology,
    entry: &Entry,
) -> crate::Result<()> {
    if let Some(desc) = &etymology.description {
        lines.push(format!("\n{}", &desc.parse(MarkdownStrategy::Disabled)));
    }

    for sense in etymology.senses.values() {
        write_sense(lines, sense, entry)?;
    }

    Ok(())
}
