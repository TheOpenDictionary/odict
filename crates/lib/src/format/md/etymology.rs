use crate::schema::{Entry, Etymology};

use super::pronunciation::write_pronunciation;
use super::sense::write_sense;

pub fn write_ety(
    lines: &mut Vec<String>,
    etymology: &Etymology,
    entry: &Entry,
) -> crate::Result<()> {
    if let Some(desc) = &etymology.description {
        lines.push(format!("\n{}", &desc));
    }

    // Add pronunciations if present
    if !etymology.pronunciations.is_empty() {
        lines.push(String::from("\n### Pronunciation"));

        for pronunciation in &etymology.pronunciations {
            write_pronunciation(lines, pronunciation, entry)?;
        }
    }

    for sense in etymology.senses.iter() {
        write_sense(lines, sense, entry)?;
    }

    Ok(())
}
