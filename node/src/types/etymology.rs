use odict::EnumIdentifier;
use std::collections::HashMap;

use super::pronunciation::Pronunciation;
use super::sense::Sense;

#[napi(object)]
pub struct Etymology {
  pub id: Option<String>,
  pub pronunciations: Vec<Pronunciation>,
  pub description: Option<String>,
  pub senses: HashMap<String, Sense>,
}

impl From<odict::Etymology> for Etymology {
  fn from(ety: odict::Etymology) -> Self {
    Self {
      id: ety.id,
      pronunciations: ety.pronunciations.into_iter().map(Into::into).collect(),
      description: ety.description,
      senses: ety
        .senses
        .into_iter()
        .map(|(k, v)| (k.id(), v.into()))
        .collect(),
    }
  }
}
