use crate::schema::{DefinitionType, Entry, Sense};

use super::{definition::write_definition, group::write_group};

pub fn write_sense(lines: &mut Vec<String>, sense: &Sense, entry: &Entry) -> crate::Result<()> {
    lines.push(format!("\n_{}_\n", sense.pos.to_string()));

    for (idx, dt) in sense.definitions.iter().enumerate() {
        match dt {
            DefinitionType::Definition(d) => write_definition(lines, idx, 2, d, entry)?,
            DefinitionType::Group(g) => write_group(lines, idx, g, entry)?,
        }
    }

    Ok(())
}
