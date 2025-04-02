use pyo3::prelude::*;

use super::{note::Note, Example};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Definition {
    #[pyo3(get)]
    pub id: Option<String>,
    #[pyo3(get)]
    pub value: String,
    #[pyo3(get)]
    pub examples: Vec<Example>,
    #[pyo3(get)]
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
