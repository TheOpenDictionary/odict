use std::collections::HashMap;

use serde::Serialize;

use crate::{Etymology, MDString, PartOfSpeech};

use super::SenseJSON;

#[derive(Serialize)]
pub struct EtymologyJSON {
    pub id: Option<String>,
    pub pronunciation: Option<String>,
    pub description: Option<MDString>,
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
