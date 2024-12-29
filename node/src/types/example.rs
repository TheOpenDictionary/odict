use napi::bindgen_prelude::*;

#[napi(object)]
pub struct Example {
  pub value: String,
}

impl Example {
  pub fn from(note: odict::Example) -> Result<Self> {
    let odict::Example { value } = note;

    Ok(Self { value })
  }
}
