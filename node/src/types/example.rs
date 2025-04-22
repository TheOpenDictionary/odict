use crate::types::{Pronunciation, Translation};
use structural_convert::StructuralConvert;

#[napi(object)]
#[derive(StructuralConvert)]
#[convert(from(odict::Example))]
pub struct Example {
  pub value: String,
  pub translations: Vec<Translation>,
  pub pronunciations: Vec<Pronunciation>,
}
