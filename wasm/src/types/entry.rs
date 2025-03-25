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

impl Entry {
    pub fn from_archive(entry: &odict::ArchivedEntry) -> Result<Self, JsValue> {
        let term = entry.term().to_string();
        let see_also = entry.see_also().map(|s| s.to_string());
        
        let etymologies = entry
            .etymologies()
            .iter()
            .map(|ety| Etymology {
                description: ety.description().map(|d| d.to_string()),
                senses: ety
                    .senses()
                    .iter()
                    .map(|(pos, sense)| Sense {
                        part_of_speech: pos.to_string(),
                        definitions: sense.definitions().iter().map(|d| d.to_string()).collect(),
                        examples: sense.examples().iter().map(|e| e.to_string()).collect(),
                    })
                    .collect(),
            })
            .collect();

        Ok(Entry {
            term,
            see_also,
            etymologies,
        })
    }

    pub fn from_entry(entry: odict::Entry) -> Result<Self, JsValue> {
        Ok(Entry {
            term: entry.term,
            see_also: entry.see_also,
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
        })
    }
}
