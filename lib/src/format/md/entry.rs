use crate::Entry;

use super::etymology::write_ety;

pub fn write_entry(entry: &Entry) -> crate::Result<String> {
    let mut lines: Vec<String> = vec![];

    lines.push(format!("\n# {}", entry.term));

    let ety_count = entry.etymologies.len();

    for (idx, etymology) in entry.etymologies.iter().enumerate() {
        if ety_count > 1 {
            lines.push(format!("## \nEtymology #{}", idx + 1));
        }

        write_ety(&mut lines, etymology, entry)?;
    }

    Ok(lines.join("\n"))
}
