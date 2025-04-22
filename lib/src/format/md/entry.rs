use crate::Entry;

use super::etymology::write_ety;
use super::pronunciation::write_pronunciation;

pub fn write_entry(entry: &Entry) -> crate::Result<String> {
    let mut lines: Vec<String> = vec![];

    lines.push(format!("\n# {}", entry.term));

    // Add pronunciations if present
    if !entry.pronunciations.is_empty() {
        lines.push(String::from("\n## Pronunciation"));

        for pronunciation in &entry.pronunciations {
            write_pronunciation(&mut lines, pronunciation, entry)?;
        }
    }

    let ety_count = entry.etymologies.len();

    for (idx, etymology) in entry.etymologies.iter().enumerate() {
        if ety_count > 1 {
            lines.push(format!("## \nEtymology #{}", idx + 1));
        }

        write_ety(&mut lines, etymology, entry)?;
    }

    Ok(lines.join("\n"))
}
