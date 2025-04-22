use crate::types::{Pronunciation, Translation};
use napi::bindgen_prelude::*;

#[napi(object)]
pub struct Example {
  pub value: String,
  #[napi(ts_type = "Translation[]")]
  pub translations: Vec<Translation>,
  #[napi(ts_type = "Pronunciation[]")]
  pub pronunciations: Vec<Pronunciation>,
}

impl Example {
  pub fn from(example: odict::Example) -> Result<Self> {
    Ok(Self {
      value: example.value,
      translations: example
        .translations
        .into_iter()
        .map(Translation::from)
        .collect::<Result<Vec<Translation>>>()?,
      pronunciations: example
        .pronunciations
        .into_iter()
        .map(Pronunciation::from)
        .collect::<Result<Vec<Pronunciation>>>()?,
    })
  }
}
