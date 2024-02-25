use serde::Serialize;

use crate::{Definition, MDString};

use super::NoteJSON;

#[derive(Serialize)]
pub struct DefinitionJSON {
    pub id: Option<String>,
    pub value: MDString,
    pub examples: Vec<String>,
    pub notes: Vec<NoteJSON>,
}

impl From<Definition> for DefinitionJSON {
    fn from(definition: Definition) -> Self {
        let Definition {
            id,
            value,
            examples,
            notes,
        } = definition;

        Self {
            id,
            value,
            examples: examples.into_iter().map(|e| e.value).collect(),
            notes: notes.into_iter().map(|n| NoteJSON::from(n)).collect(),
        }
    }
}
