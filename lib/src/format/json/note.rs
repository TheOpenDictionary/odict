use serde::Serialize;

use crate::{Example, Note};

#[derive(Serialize)]
pub struct NoteJSON {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    value: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    examples: Vec<Example>,
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
            examples,
        }
    }
}
