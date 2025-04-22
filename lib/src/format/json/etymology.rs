use std::collections::HashMap;

use serde::Serialize;

use crate::{Etymology, PartOfSpeech};

use super::{ordered_map, PronunciationJSON, SenseJSON};

#[derive(Serialize)]
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

impl From<Etymology> for EtymologyJSON {
    fn from(etymology: Etymology) -> Self {
        let Etymology {
            id,
            pronunciations,
            description,
            senses,
        } = etymology;

        Self {
            id,
            pronunciations: pronunciations
                .into_iter()
                .map(PronunciationJSON::from)
                .collect(),
            description,
            senses: senses
                .into_iter()
                .map(|(k, v)| (k, SenseJSON::from(v)))
                .collect(),
        }
    }
}
