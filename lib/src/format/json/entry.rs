use std::hash::Hasher;

use crate::schema::{ArchivedEntry, Entry};
use structural_convert::StructuralConvert;

use serde::Serialize;

use super::{EntryRefJSON, EtymologyJSON};

#[derive(Serialize, Eq, PartialEq, StructuralConvert)]
#[convert(from(Entry))]
pub struct EntryJSON {
    pub term: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<EntryRefJSON>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub etymologies: Vec<EtymologyJSON>,
}

impl std::hash::Hash for EntryJSON {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.term.hash(state)
    }
}

impl TryFrom<&ArchivedEntry> for EntryJSON {
    fn try_from(entry: &ArchivedEntry) -> crate::Result<Self> {
        Ok(entry.deserialize()?.into())
    }
    type Error = crate::Error;
}
