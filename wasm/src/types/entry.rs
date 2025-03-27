use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Entry {
    term: String,
    see_also: Option<String>,
    etymologies: Vec<Etymology>,
}

#[wasm_bindgen]
impl Entry {
    #[wasm_bindgen(getter)]
    pub fn term(&self) -> String {
        self.term.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_term(&mut self, term: String) {
        self.term = term;
    }

    #[wasm_bindgen(getter)]
    pub fn see_also(&self) -> Option<String> {
        self.see_also.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_see_also(&mut self, see_also: Option<String>) {
        self.see_also = see_also;
    }

    #[wasm_bindgen(getter)]
    pub fn etymologies(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.etymologies).unwrap()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Etymology {
    description: Option<String>,
    senses: Vec<Sense>,
}

#[wasm_bindgen]
impl Etymology {
    #[wasm_bindgen(getter)]
    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    #[wasm_bindgen(getter)]
    pub fn senses(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.senses).unwrap()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Sense {
    part_of_speech: String,
    definitions: Vec<String>,
    examples: Vec<String>,
}

#[wasm_bindgen]
impl Sense {
    #[wasm_bindgen(getter)]
    pub fn part_of_speech(&self) -> String {
        self.part_of_speech.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_part_of_speech(&mut self, part_of_speech: String) {
        self.part_of_speech = part_of_speech;
    }

    #[wasm_bindgen(getter)]
    pub fn definitions(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.definitions).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn examples(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.examples).unwrap()
    }
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
                .into_iter()  // Use into_iter() to take ownership
                .map(|ety| Etymology {
                    description: ety.description,
                    senses: ety
                        .senses
                        .into_iter()  // Use into_iter() to take ownership
                        .map(|(pos, sense)| Sense {
                            part_of_speech: pos,
                            definitions: sense.definitions,
                            examples: sense.examples,
                        })
                        .collect(),
                })
                .collect(),
        })
    }
}
