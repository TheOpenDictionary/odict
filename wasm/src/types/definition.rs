use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::{note::Note, Example};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Definition {
  id: Option<String>,
  value: String,
  examples: Vec<Example>,
  notes: Vec<Note>,
}

#[wasm_bindgen]
impl Definition {
  #[wasm_bindgen(getter)]
  pub fn id(&self) -> Option<String> {
    self.id.clone()
  }

  #[wasm_bindgen(getter)]
  pub fn value(&self) -> String {
    self.value.clone()
  }

  #[wasm_bindgen(getter)]
  pub fn examples(&self) -> JsValue {
    serde_wasm_bindgen::to_value(&self.examples).unwrap()
  }

  #[wasm_bindgen(getter)]
  pub fn notes(&self) -> JsValue {
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
