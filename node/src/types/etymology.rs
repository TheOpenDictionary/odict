use std::collections::HashMap;
use structural_convert::StructuralConvert;

use super::pronunciation::Pronunciation;
use super::sense::Sense;

#[napi(object)]
#[derive(StructuralConvert)]
#[convert(from(odict::Etymology))]
pub struct Etymology {
  pub id: Option<String>,
  pub pronunciations: Vec<Pronunciation>,
  pub description: Option<String>,
  pub senses: HashMap<String, Sense>,
}
