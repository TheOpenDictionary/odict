use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub term: String,
    pub see_also: Option<String>,
    pub etymologies: Vec<Etymology>,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Etymology {
    pub description: Option<String>,
    pub senses: Vec<Sense>,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Sense {
    pub part_of_speech: String,
    pub definitions: Vec<String>,
    pub examples: Vec<String>,
}

impl From<&odict::models::Entry> for Entry {
    fn from(entry: &odict::models::Entry) -> Self {
        Entry {
            term: entry.term.clone(),
            see_also: entry.see_also.clone(),
            etymologies: entry
                .etymologies
                .iter()
                .map(|ety| Etymology {
                    description: ety.description.clone(),
                    senses: ety
                        .senses
                        .iter()
                        .map(|(pos, sense)| Sense {
                            part_of_speech: pos.to_string(),
                            definitions: sense.definitions.clone(),
                            examples: sense.examples.clone(),
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}
