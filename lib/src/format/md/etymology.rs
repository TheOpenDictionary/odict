use crate::{Entry, Etymology};

use super::sense::write_sense;

pub fn write_ety(
    lines: &mut Vec<String>,
    etymology: &Etymology,
    entry: &Entry,
) -> crate::Result<()> {
    if let Some(desc) = &etymology.description {
        lines.push(format!("\n{}", &desc));
    }

    for sense in etymology.senses.values() {
        write_sense(lines, sense, entry)?;
    }

    Ok(())
}
