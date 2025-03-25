use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Example {
  pub value: String,
}

impl Example {
  pub fn from(note: odict::Example) -> Result<Self, JsValue> {
    let odict::Example { value } = note;

    Ok(Self { value })
  }
}
