use pyo3::prelude::*;

use super::{mdstring::MDString, Example};

#[pyclass]
pub struct Note {
    pub id: Option<String>,
    pub value: MDString,
    pub examples: Vec<Example>,
}

impl Note {
    pub fn from(note: odict::Note) -> PyResult<Self> {
        let odict::Note {
            id,
            value,
            examples,
        } = note;

        Ok(Self {
            id,
            value: MDString::from(value),
            examples: examples
                .into_iter()
                .map(|e| Example::from(e))
                .collect::<Result<Vec<Example>, _>>()?,
        })
    }
}
