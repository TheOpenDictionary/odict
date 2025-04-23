use crate::{ArchivedEntry, Entry};
use structural_convert::StructuralConvert;

use serde::Serialize;

use super::{EntryRefJSON, EtymologyJSON};

#[derive(Serialize, StructuralConvert)]
#[convert(from(Entry))]
pub struct EntryJSON {
    pub term: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<EntryRefJSON>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub etymologies: Vec<EtymologyJSON>,
}

impl TryFrom<&ArchivedEntry> for EntryJSON {
    fn try_from(entry: &ArchivedEntry) -> crate::Result<Self> {
        Ok(entry.to_entry()?.into())
    }
    type Error = crate::Error;
}
