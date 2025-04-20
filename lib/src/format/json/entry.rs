use crate::{ArchivedEntry, Entry};

use serde::Serialize;

use super::{EntryRefJSON, EtymologyJSON, FormJSON};

#[derive(Serialize)]
pub struct EntryJSON {
    pub term: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<EntryRefJSON>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lemma: Option<EntryRefJSON>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub etymologies: Vec<EtymologyJSON>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub forms: Vec<FormJSON>,
}

impl From<Entry> for EntryJSON {
    fn from(entry: Entry) -> Self {
        let Entry {
            term,
            see_also,
            lemma,
            etymologies,
            forms,
        } = entry;

        Self {
            term,
            see_also: see_also.map(EntryRefJSON::from),
            lemma: lemma.map(EntryRefJSON::from),
            etymologies: etymologies
                .into_iter()
                .map(|e| EtymologyJSON::from(e))
                .collect(),
            forms: forms.into_iter().map(|f| FormJSON::from(f)).collect(),
        }
    }
}

impl From<&ArchivedEntry> for EntryJSON {
    fn from(entry: &ArchivedEntry) -> Self {
        EntryJSON::from(entry.to_entry().unwrap())
    }
}
