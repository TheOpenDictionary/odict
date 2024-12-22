use pyo3::prelude::*;

use super::mdstring::MDString;

#[pyclass]
pub struct Example {
    pub value: MDString,
}

impl Example {
    pub fn from(note: odict::Example) -> PyResult<Self> {
        let odict::Example { value } = note;

        Ok(Self {
            value: MDString::from(value),
        })
    }
}
