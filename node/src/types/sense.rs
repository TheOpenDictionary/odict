use napi::bindgen_prelude::*;

use odict::DefinitionType;
use structural_convert::StructuralConvert;

use super::{definition::Definition, group::Group};

#[napi(object)]
#[derive(StructuralConvert)]
#[convert(from(odict::Sense))]
pub struct Sense {
  pub pos: String,
  pub lemma: Option<String>,
  pub definitions: Vec<Either<Definition, Group>>,
  pub tags: Vec<String>,
}

impl From<DefinitionType> for Either<Definition, Group> {
  fn from(definition_type: DefinitionType) -> Self {
    match definition_type {
      DefinitionType::Definition(def) => Either::A(def.into()),
      DefinitionType::Group(group) => Either::B(group.into()),
    }
  }
}
