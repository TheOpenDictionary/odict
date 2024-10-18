use serde::Serialize;

use crate::{MDString, Note};

#[derive(Serialize)]
pub struct NoteJSON {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    value: MDString,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    examples: Vec<MDString>,
}

impl From<Note> for NoteJSON {
    fn from(note: Note) -> Self {
        let Note {
            id,
            value,
            examples,
        } = note;

        Self {
            id,
            value,
            examples: examples.into_iter().map(|e| e.value).collect(),
        }
    }
}
