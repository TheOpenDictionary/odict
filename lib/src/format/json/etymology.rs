use indexmap::IndexSet;

use serde::Serialize;

use crate::schema::Etymology;

use super::{PronunciationJSON, SenseJSON};

#[derive(Serialize, PartialEq, Eq)]
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

impl From<Etymology> for EtymologyJSON {
    fn from(etymology: Etymology) -> Self {
        Self {
            id: etymology.id,
            pronunciations: etymology
                .pronunciations
                .into_iter()
                .map(Into::into)
                .collect(),
            description: etymology.description,
            senses: etymology.senses.into_iter().map(Into::into).collect(),
        }
    }
}
