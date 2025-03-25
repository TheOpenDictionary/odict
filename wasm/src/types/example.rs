use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Example {
  pub value: String,
}

impl Example {
  pub fn from(note: odict::Example) -> Result<Self, JsValue> {
    let odict::Example { value } = note;

    Ok(Self { value })
  }
}
