use serde::Serialize;

use crate::{lookup::Token, ArchivedEntry, Entry};

use super::EntryJSON;

#[derive(Serialize)]
pub struct TokenJSON {
    pub lemma: String,
    pub language: Option<String>,
    pub script: String,
    pub kind: String,
    pub start: usize,
    pub end: usize,
    pub entries: Vec<EntryJSON>,
}

impl From<Token<Entry>> for TokenJSON {
    fn from(token: Token<Entry>) -> Self {
        let Token {
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
            entries: entries
                .into_iter()
                .map(|result| EntryJSON::from(result.entry))
                .collect(),
        }
    }
}

impl From<Token<&ArchivedEntry>> for TokenJSON {
    fn from(token: Token<&ArchivedEntry>) -> Self {
        let Token {
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
            entries: entries
                .into_iter()
                .map(|result| EntryJSON::from(result.entry))
                .collect(),
        }
    }
}
