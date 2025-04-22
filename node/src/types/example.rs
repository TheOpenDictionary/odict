use crate::types::Translation;
use napi::bindgen_prelude::*;

#[napi(object)]
pub struct Example {
  pub value: String,
  #[napi(ts_type = "Translation[]")]
  pub translations: Vec<Translation>,
}

impl Example {
  pub fn from(example: odict::Example) -> Result<Self> {
    let odict::Example {
      value,
      translations,
    } = example;

    let mapped_translations = translations
      .into_iter()
      .map(Translation::from)
      .collect::<Result<Vec<Translation>>>()?;

    Ok(Self {
      value,
      translations: mapped_translations,
    })
  }
}
