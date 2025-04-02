use serde::Serialize;

use crate::{ArchivedEntry, Entry, lookup::Token};

use super::EntryJSON;

#[derive(Serialize)]
pub struct TokenJSON {
    pub lemma: String,
    pub language: Option<String>,
    pub entries: Vec<EntryJSON>,
}

impl From<Token<Entry>> for TokenJSON {
    fn from(token: Token<Entry>) -> Self {
        let Token {
            lemma,
            language,
            entries,
        } = token;

        Self {
            lemma,
            language,
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
        } = token;

        Self {
            lemma,
            language,
            entries: entries
                .into_iter()
                .map(|result| EntryJSON::from(result.entry))
                .collect(),
        }
    }
}
