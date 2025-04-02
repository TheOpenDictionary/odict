use napi::bindgen_prelude::*;

use super::LookupResult;

#[napi(object)]
pub struct Token {
  pub lemma: String,
  pub language: Option<String>,
  pub entries: Vec<LookupResult>,
}
