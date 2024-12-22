use pyo3::prelude::*;

use super::{mdstring::MDString, note::Note, Example};

#[pyclass]
pub struct Definition {
    pub id: Option<String>,
    pub value: MDString,
    pub examples: Vec<Example>,
    pub notes: Vec<Note>,
}

impl Definition {
    pub fn from(definition: odict::Definition) -> PyResult<Self> {
        let odict::Definition {
            id,
            value,
            examples,
            notes,
        } = definition;

        Ok(Self {
            id,
            value: MDString::from(value),
            examples: examples
                .into_iter()
                .map(|e| Example::from(e).unwrap())
                .collect::<Vec<Example>>(),
            notes: notes.into_iter().map(|n| Note::from(n).unwrap()).collect(),
        })
    }
}
