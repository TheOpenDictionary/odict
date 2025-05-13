use std::collections::{BTreeMap, HashSet};
use structural_convert::StructuralConvert;

use serde::{Serialize, Serializer};

use crate::{Dictionary, ID};

use super::EntryJSON;

#[derive(Serialize, StructuralConvert)]
#[convert(from(Dictionary))]
pub struct DictionaryJSON {
    pub id: ID,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        serialize_with = "hash_entries",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub entries: HashSet<EntryJSON>,
}

pub fn hash_entries<S>(value: &HashSet<EntryJSON>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut ordered_map = BTreeMap::new();

    for item in value {
        ordered_map.insert(item.term.to_string(), item);
    }

    ordered_map.serialize(serializer)
}
