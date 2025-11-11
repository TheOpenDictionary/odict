use indexmap::IndexSet;
use structural_convert::StructuralConvert;

use serde::Serialize;

use crate::schema::Etymology;

use super::{PronunciationJSON, SenseJSON};

#[derive(Serialize, PartialEq, Eq, StructuralConvert)]
#[convert(from(Etymology))]
pub struct EtymologyJSON {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pronunciations: Vec<PronunciationJSON>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "indexmap::IndexSet::is_empty")]
    pub senses: IndexSet<SenseJSON>,
}
