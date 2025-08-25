use internal::ToEnumWrapper;
use napi::bindgen_prelude::*;

use odict::schema::DefinitionType;

use super::{
  definition::Definition, enums::EnumWrapper, form::Form, group::Group, translation::Translation,
};

#[napi(object)]
pub struct Sense {
  pub pos: EnumWrapper,
  pub lemma: Option<String>,
  pub definitions: Vec<Either<Definition, Group>>,
  pub tags: Vec<String>,
  pub translations: Vec<Translation>,
  pub forms: Vec<Form>,
}

impl From<odict::Sense> for Sense {
  fn from(sense: odict::Sense) -> Self {
    Sense {
      pos: sense.pos.to_enum_wrapper().into(),
      lemma: sense.lemma.map(|entry_ref| entry_ref.to_string()),
      definitions: sense
        .definitions
        .into_iter()
        .map(|def_type| match def_type {
          DefinitionType::Definition(def) => Either::A(def.into()),
          DefinitionType::Group(group) => Either::B(group.into()),
        })
        .collect(),
      tags: sense.tags,
      translations: sense.translations.into_iter().map(|t| t.into()).collect(),
      forms: sense.forms.into_iter().map(|f| f.into()).collect(),
    }
  }
}
