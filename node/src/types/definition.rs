use structural_convert::StructuralConvert;

use super::{note::Note, Example};

#[derive(StructuralConvert)]
#[convert(from(odict::Definition))]
#[napi(object)]
pub struct Definition {
  pub id: Option<String>,
  pub value: String,
  pub examples: Vec<Example>,
  pub notes: Vec<Note>,
}
