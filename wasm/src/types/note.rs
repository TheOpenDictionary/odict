use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::Example;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Note {
  #[wasm_bindgen(skip)]
  pub id: Option<String>,
  #[wasm_bindgen(skip)]
  pub value: String,
  #[wasm_bindgen(skip)]
  pub examples: Vec<Example>,
}

#[wasm_bindgen]
impl Note {
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
