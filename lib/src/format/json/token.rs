use serde::Serialize;

use crate::{lookup::Token, ArchivedEntry};

use super::EntryJSON;

#[derive(Serialize)]
pub struct TokenJSON {
    pub lemma: String,
    pub language: String,
    pub entries: Vec<EntryJSON>,
}

impl From<Token<'_>> for TokenJSON {
    fn from(token: Token) -> Self {
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

impl From<Token<'_, &ArchivedEntry>> for TokenJSON {
    fn from(token: Token<'_, &ArchivedEntry>) -> Self {
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
