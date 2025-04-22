use napi::bindgen_prelude::*;

#[napi(object)]
pub struct Translation {
  pub lang: String,
  pub value: String,
}

impl Translation {
  pub fn from(translation: odict::Translation) -> Result<Self> {
    let odict::Translation { lang, value } = translation;

    Ok(Self { lang, value })
  }
}
