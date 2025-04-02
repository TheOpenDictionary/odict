use pyo3::prelude::*;

use super::LookupResult;

#[pyclass]
#[derive(Debug)]
pub struct Token {
    pub lemma: String,
    pub language: Option<String>,
    pub entries: Vec<LookupResult>,
}

#[pymethods]
impl Token {
    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    pub fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}
