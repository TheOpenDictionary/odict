use crate::Entry;

use serde::Serialize;

use super::EtymologyJSON;

#[derive(Serialize)]
pub struct EntryJSON {
    pub term: String,
    pub see_also: Option<String>,
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
