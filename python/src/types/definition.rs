use pyo3::prelude::*;

use super::{note::Note, Example};

#[pyclass]
#[derive(Debug)]
pub struct Definition {
    pub id: Option<String>,
    pub value: String,
    pub examples: Vec<Example>,
    pub notes: Vec<Note>,
}

impl From<odict::Definition> for Definition {
    fn from(definition: odict::Definition) -> Self {
        let odict::Definition {
            id,
            value,
            examples,
            notes,
        } = definition;

        Self {
            id,
            value: String::from(value),
            examples: examples.into_iter().map(Example::from).collect(),
            notes: notes.into_iter().map(Note::from).collect(),
        }
    }
}
