use std::collections::HashMap;

use serde::Serialize;

use crate::{Dictionary, ID};

use super::{ordered_map, EntryJSON};

#[derive(Serialize)]
pub struct DictionaryJSON {
    pub id: ID,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        serialize_with = "ordered_map",
        skip_serializing_if = "HashMap::is_empty"
    )]
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
