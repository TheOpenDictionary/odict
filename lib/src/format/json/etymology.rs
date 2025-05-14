use std::collections::{BTreeMap, HashSet};
use structural_convert::StructuralConvert;

use serde::{Serialize, Serializer};

use crate::{EnumIdentifier, Etymology};

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

    #[serde(
        serialize_with = "hash_senses",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub senses: HashSet<SenseJSON>,
}

pub fn hash_senses<S>(value: &HashSet<SenseJSON>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut ordered_map = BTreeMap::new();

    for item in value {
        ordered_map.insert(item.pos.id(), item);
    }

    ordered_map.serialize(serializer)
}
