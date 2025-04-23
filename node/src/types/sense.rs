use napi::bindgen_prelude::*;

use odict::DefinitionType;

use super::{definition::Definition, group::Group};

#[napi(object)]
pub struct Sense {
  pub pos: String,
  pub lemma: Option<String>,
  pub definitions: Vec<Either<Definition, Group>>,
  pub tags: Vec<String>,
}

impl From<odict::Sense> for Sense {
  fn from(sense: odict::Sense) -> Self {
    Sense {
      pos: sense.pos.to_string(),
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
    }
  }
}
