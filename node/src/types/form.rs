use napi::bindgen_prelude::*;
use structural_convert::StructuralConvert;

use super::form_kind::FormKind;

#[napi(object)]
#[derive(Clone, StructuralConvert)]
#[convert(from(odict::Form))]
pub struct Form {
  pub term: String,
  pub kind: Option<FormKind>,
  pub tags: Vec<String>,
}
