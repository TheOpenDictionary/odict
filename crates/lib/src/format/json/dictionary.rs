use indexmap::IndexSet;

use serde::Serialize;

use crate::schema::{Dictionary, ID};

use super::EntryJSON;

#[derive(Serialize)]
pub struct DictionaryJSON {
    pub id: ID,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "indexmap::IndexSet::is_empty")]
    pub entries: IndexSet<EntryJSON>,
}

impl From<Dictionary> for DictionaryJSON {
    fn from(dictionary: Dictionary) -> Self {
        Self {
            id: dictionary.id,
            name: dictionary.name,
            entries: dictionary.entries.into_iter().map(Into::into).collect(),
        }
    }
}
