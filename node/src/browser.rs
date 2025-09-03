use napi::bindgen_prelude::*;

use crate::types::{self};

#[napi]
pub struct OpenDictionary {
  dict: odict::OpenDictionary,
}

#[napi]
impl OpenDictionary {
  #[napi(constructor)]
  pub fn new(data: Buffer) -> Result<Self> {
    let dict = crate::shared::new_from_bytes(&data)?;
    Ok(Self { dict })
  }

  #[napi(getter)]
  pub fn min_rank(&self) -> Result<Option<u32>> {
    crate::shared::get_min_rank(&self.dict)
  }

  #[napi(getter)]
  pub fn max_rank(&self) -> Result<Option<u32>> {
    crate::shared::get_max_rank(&self.dict)
  }

  #[napi]
  pub fn lookup(
    &self,
    query: Either<String, Vec<String>>,
    options: Option<types::LookupOptions>,
  ) -> Result<Vec<types::LookupResult>> {
    crate::shared::perform_lookup(&self.dict, query, options)
  }

  #[napi]
  pub fn lexicon(&self) -> Result<Vec<&str>> {
    crate::shared::get_lexicon(&self.dict)
  }

  #[napi]
  pub fn tokenize(
    &self,
    text: String,
    options: Option<types::TokenizeOptions>,
  ) -> Result<Vec<types::Token>> {
    crate::shared::perform_tokenization(&self.dict, &text, options)
  }
}
