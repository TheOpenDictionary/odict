use crate::types::{Pronunciation, Translation};
use napi::bindgen_prelude::*;
use structural_convert::StructuralConvert;

#[napi(object)]
#[derive(StructuralConvert)]
#[convert(from(odict::Example))]
pub struct Example {
  pub value: String,
  #[napi(ts_type = "Translation[]")]
  pub translations: Vec<Translation>,
  #[napi(ts_type = "Pronunciation[]")]
  pub pronunciations: Vec<Pronunciation>,
}
