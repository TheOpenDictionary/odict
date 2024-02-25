use serde::Serialize;

use crate::{MDString, Note};

#[derive(Serialize)]
pub struct NoteJSON {
    id: Option<String>,
    value: MDString,
    examples: Vec<String>,
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
