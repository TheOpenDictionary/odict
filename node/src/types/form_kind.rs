use napi::bindgen_prelude::*;

use structural_convert::StructuralConvert;

#[napi]
#[derive(StructuralConvert)]
#[convert(from(odict::FormKind))]
pub enum FormKind {
  Conjugation,
  Inflection,
  Plural,
  Singular,
  Comparative,
  Superlative,
  Other,
}
