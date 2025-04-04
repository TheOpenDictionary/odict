use odict::{ArchivedEntry, Entry};
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
    #[pyo3(get)]
    pub kind: String,
    #[pyo3(get)]
    pub script: String,
    #[pyo3(get)]
    pub start: usize,
    #[pyo3(get)]
    pub end: usize,
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
            kind,
            script,
            start,
            end,
        } = token;

        Self {
            lemma,
            language: language.map(|lang| lang.code().to_string()),
            script: script.name().to_string(),
            kind: format!("{:?}", kind),
            start,
            end,
            entries: entries.into_iter().map(LookupResult::from).collect(),
        }
    }
}

impl From<odict::Token<&ArchivedEntry>> for Token {
    fn from(token: odict::Token<&ArchivedEntry>) -> Self {
        let odict::Token {
            lemma,
            language,
            entries,
            kind,
            script,
            start,
            end,
        } = token;

        Self {
            lemma,
            language: language.map(|lang| lang.code().to_string()),
            script: script.name().to_string(),
            kind: format!("{:?}", kind),
            start,
            end,
            entries: entries.into_iter().map(LookupResult::from).collect(),
        }
    }
}
