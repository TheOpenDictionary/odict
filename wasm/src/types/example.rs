use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Example {
  value: String,
}

#[wasm_bindgen]
impl Example {
  #[wasm_bindgen(getter)]
  pub fn value(&self) -> String {
    self.value.clone()
  }

  #[wasm_bindgen(setter)]
  pub fn set_value(&mut self, value: String) {
    self.value = value;
  }
}

impl Example {
  pub fn from(note: odict::Example) -> Result<Self, JsValue> {
    let odict::Example { value } = note;

    Ok(Self { value })
  }
}
