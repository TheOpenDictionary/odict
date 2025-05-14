use std::collections::HashMap;
use structural_convert::StructuralConvert;

use serde::Serialize;

use crate::{Etymology, PartOfSpeech};

use super::{ordered_map, PronunciationJSON, SenseJSON};

#[derive(Serialize, StructuralConvert)]
#[convert(from(Etymology))]
pub struct EtymologyJSON {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pronunciations: Vec<PronunciationJSON>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(
        serialize_with = "ordered_map",
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub senses: HashMap<PartOfSpeech, SenseJSON>,
}
