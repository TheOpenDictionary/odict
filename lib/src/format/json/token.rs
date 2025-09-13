use serde::Serialize;

use crate::{
    schema::{ArchivedEntry, Entry},
    tokenize::Token,
};

use super::EntryJSON;

#[derive(Serialize, PartialEq, Eq)]
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
            kind: format!("{kind:?}"),
            start,
            end,
            entries: entries
                .into_iter()
                .map(|result| EntryJSON::from(result.entry))
                .collect(),
        }
    }
}

impl TryFrom<Token<&ArchivedEntry>> for TokenJSON {
    type Error = crate::Error;

    fn try_from(token: Token<&ArchivedEntry>) -> crate::Result<Self> {
        let Token {
            lemma,
            language,
            entries,
            kind,
            script,
            start,
            end,
        } = token;

        Ok(Self {
            lemma,
            language: language.map(|lang| lang.code().to_string()),
            script: script.name().to_string(),
            kind: format!("{kind:?}"),
            start,
            end,
            entries: entries
                .into_iter()
                .map(|result| EntryJSON::try_from(result.entry))
                .collect::<crate::Result<Vec<EntryJSON>>>()?,
        })
    }
}
