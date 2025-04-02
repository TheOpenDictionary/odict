use pyo3::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct Example {
    pub value: String,
}

impl From<odict::Example> for Example {
    fn from(example: odict::Example) -> Self {
        let odict::Example { value } = example;

        Self {
            value: String::from(value),
        }
    }
}
