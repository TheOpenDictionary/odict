use odict::Entry;
use pyo3::prelude::*;

use super::LookupResult;

#[pyclass]
#[derive(Debug)]
pub struct Token {
    pub lemma: String,
    pub language: Option<String>,
    pub entries: Vec<LookupResult>,
}

impl Token {
    pub fn from(note: odict::Token<Entry>) -> PyResult<Self> {
        let odict::Token {
            lemma,
            language,
            entries,
        } = note;

        Ok(Self {
            lemma,
            language,
            entries: entries
                .into_iter()
                .map(|e| LookupResult::from(e))
                .collect::<Result<Vec<LookupResult>, _>>()?,
        })
    }
}
