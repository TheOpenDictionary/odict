use pyo3::prelude::*;

use super::Example;

#[pyclass]
#[derive(Debug)]
pub struct Note {
    pub id: Option<String>,
    pub value: String,
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
            value: String::from(value),
            examples: examples
                .into_iter()
                .map(|e| Example::from(e))
                .collect::<Result<Vec<Example>, _>>()?,
        })
    }
}
