use pyo3::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct Example {
    pub value: String,
}

impl Example {
    pub fn from(note: odict::Example) -> PyResult<Self> {
        let odict::Example { value } = note;

        Ok(Self {
            value: String::from(value),
        })
    }
}
