use std::collections::HashMap;

use serde::Serialize;

use crate::{Dictionary, ID};

use super::EntryJSON;

#[derive(Serialize)]
pub struct DictionaryJSON {
    pub id: ID,
    pub name: Option<String>,
    pub entries: HashMap<String, EntryJSON>,
}

impl From<Dictionary> for DictionaryJSON {
    fn from(dictionary: Dictionary) -> Self {
        let Dictionary { id, name, entries } = dictionary;

        Self {
            id,
            name,
            entries: entries
                .into_iter()
                .map(|(k, v)| (k, EntryJSON::from(v)))
                .collect(),
        }
    }
}
