use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::Example;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Note {
  id: Option<String>,
  value: String,
  examples: Vec<Example>,
}

#[wasm_bindgen]
impl Note {
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
}

impl Note {
  pub fn from(note: odict::Note) -> Result<Self, JsValue> {
    let odict::Note {
      id,
      value,
      examples,
    } = note;

    Ok(Self {
      id,
      value,
      examples: examples
        .into_iter()
        .map(|e| Example::from(e))
        .collect::<Result<Vec<Example>, _>>()?,
    })
  }
}
