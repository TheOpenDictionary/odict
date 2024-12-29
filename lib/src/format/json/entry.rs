use crate::{ArchivedEntry, Entry};

use serde::Serialize;

use super::EtymologyJSON;

#[derive(Serialize)]
pub struct EntryJSON {
    pub term: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub etymologies: Vec<EtymologyJSON>,
}

impl From<Entry> for EntryJSON {
    fn from(entry: Entry) -> Self {
        let Entry {
            term,
            see_also,
            etymologies,
        } = entry;

        Self {
            term,
            see_also,
            etymologies: etymologies
                .into_iter()
                .map(|e| EtymologyJSON::from(e))
                .collect(),
        }
    }
}

impl From<&ArchivedEntry> for EntryJSON {
    fn from(entry: &ArchivedEntry) -> Self {
        EntryJSON::from(entry.to_entry().unwrap())
    }
}
