use indexmap::IndexSet;

use structural_convert::StructuralConvert;

use serde::Serialize;

use crate::schema::{Dictionary, ID};

use super::EntryJSON;

#[derive(Serialize, StructuralConvert)]
#[convert(from(Dictionary))]
pub struct DictionaryJSON {
    pub id: ID,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "indexmap::IndexSet::is_empty")]
    pub entries: IndexSet<EntryJSON>,
}
