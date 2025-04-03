use odict::Entry;
use pyo3::prelude::*;

use super::LookupResult;

#[pyclass]
#[derive(Debug)]
pub struct Token {
    #[pyo3(get)]
    pub lemma: String,
    #[pyo3(get)]
    pub language: Option<String>,
    #[pyo3(get)]
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

impl From<odict::Token<Entry>> for Token {
    fn from(token: odict::Token<Entry>) -> Self {
        let odict::Token {
            lemma,
            language,
            entries,
        } = token;

        Self {
            lemma,
            language,
            entries: entries.into_iter().map(LookupResult::from).collect(),
        }
    }
}
