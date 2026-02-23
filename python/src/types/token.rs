use odict::schema::{ArchivedEntry, Entry};
use pyo3::prelude::*;

use super::LookupResult;

/// A token produced by NLP-based text segmentation.
///
/// Each token represents a segment of the input text, with metadata about
/// its position, detected language and script, and any matching dictionary entries.
#[pyclass]
#[derive(Debug)]
pub struct Token {
    /// The original token text (lemma form).
    #[pyo3(get)]
    pub lemma: String,
    /// Detected language code (e.g. `"eng"`), or `None` if unknown.
    #[pyo3(get)]
    pub language: Option<String>,
    /// Matched dictionary entries for this token.
    #[pyo3(get)]
    pub entries: Vec<LookupResult>,
    /// The token kind (e.g. `"Word"`, `"Punctuation"`).
    #[pyo3(get)]
    pub kind: String,
    /// Detected script name (e.g. `"Latin"`, `"Han"`).
    #[pyo3(get)]
    pub script: String,
    /// Start byte offset in the original text.
    #[pyo3(get)]
    pub start: usize,
    /// End byte offset in the original text.
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

impl From<odict::tokenize::Token<Entry>> for Token {
    fn from(token: odict::tokenize::Token<Entry>) -> Self {
        let odict::tokenize::Token {
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

impl From<odict::tokenize::Token<&ArchivedEntry>> for Token {
    fn from(token: odict::tokenize::Token<&ArchivedEntry>) -> Self {
        let odict::tokenize::Token {
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
