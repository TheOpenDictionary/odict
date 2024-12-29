use std::collections::HashMap;

use serde::Serialize;

use crate::{Etymology, PartOfSpeech};

use super::{ordered_map, SenseJSON};

#[derive(Serialize)]
pub struct EtymologyJSON {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(
        serialize_with = "ordered_map",
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub senses: HashMap<PartOfSpeech, SenseJSON>,
}

impl From<Etymology> for EtymologyJSON {
    fn from(entry: Etymology) -> Self {
        let Etymology {
            id,
            pronunciation,
            description,
            senses,
        } = entry;

        Self {
            id,
            pronunciation,
            description,
            senses: senses
                .into_iter()
                .map(|(k, v)| (k, SenseJSON::from(v)))
                .collect(),
        }
    }
}
