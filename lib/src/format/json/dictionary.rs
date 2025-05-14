use std::collections::HashMap;
use structural_convert::StructuralConvert;

use serde::Serialize;

use crate::{Dictionary, ID};

use super::{ordered_map, EntryJSON};

#[derive(Serialize, StructuralConvert)]
#[convert(from(Dictionary))]
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
