use napi::bindgen_prelude::*;
use structural_convert::StructuralConvert;

use super::definition::Definition;

#[napi(object)]
#[derive(StructuralConvert)]
#[convert(from(odict::Group))]
pub struct Group {
  pub id: Option<String>,
  pub description: String,
  pub definitions: Vec<Definition>,
}
