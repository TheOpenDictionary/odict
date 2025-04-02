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
            entries: entries
                .into_iter()
                .map(LookupResult::from)
                .collect(),
        }
    }
}
