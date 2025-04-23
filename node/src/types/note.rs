use structural_convert::StructuralConvert;

use super::Example;

#[napi(object)]
#[derive(StructuralConvert)]
#[convert(from(odict::Note))]
pub struct Note {
  pub id: Option<String>,
  pub value: String,
  pub examples: Vec<Example>,
}
