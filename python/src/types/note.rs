use pyo3::prelude::*;

use super::Example;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Note {
    #[pyo3(get)]
    pub id: Option<String>,
    #[pyo3(get)]
    pub value: String,
    #[pyo3(get)]
    pub examples: Vec<Example>,
}

impl From<odict::Note> for Note {
    fn from(note: odict::Note) -> Self {
        let odict::Note {
            id,
            value,
            examples,
        } = note;

        Self {
            id,
            value: String::from(value),
            examples: examples.into_iter().map(|e| Example::from(e)).collect(),
        }
    }
}
