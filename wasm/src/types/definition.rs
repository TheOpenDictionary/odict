use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::{note::Note, Example};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Definition {
  #[wasm_bindgen(skip)]
  pub id: Option<String>,
  #[wasm_bindgen(skip)]
  pub value: String,
  #[wasm_bindgen(skip)]
  pub examples: Vec<Example>,
  #[wasm_bindgen(skip)]
  pub notes: Vec<Note>,
}

#[wasm_bindgen]
impl Definition {
  #[wasm_bindgen(getter)]
  pub fn get_id(&self) -> Option<String> {
    self.id.clone()
  }

  #[wasm_bindgen(getter)]
  pub fn get_value(&self) -> String {
    self.value.clone()
  }

  #[wasm_bindgen(getter)]
  pub fn get_examples(&self) -> JsValue {
    serde_wasm_bindgen::to_value(&self.examples).unwrap()
  }

  #[wasm_bindgen(getter)]
  pub fn get_notes(&self) -> JsValue {
    serde_wasm_bindgen::to_value(&self.notes).unwrap()
  }
}

impl Definition {
  pub fn from(definition: odict::Definition) -> Result<Self, JsValue> {
    let odict::Definition {
      id,
      value,
      examples,
      notes,
    } = definition;

    Ok(Self {
      id,
      value,
      examples: examples
        .into_iter()
        .map(|e| Example::from(e).unwrap())
        .collect::<Vec<Example>>(),
      notes: notes.into_iter().map(|n| Note::from(n).unwrap()).collect(),
    })
  }
}
