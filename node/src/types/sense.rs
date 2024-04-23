use napi::Either;

use super::{definition::Definition, group::Group};

#[napi(object)]
pub struct Sense {
  pub pos: String,
  pub definitions: Vec<Either<Definition, Group>>,
}

impl Sense {
  pub fn from(sense: odict::Sense, mds: &odict::MarkdownStrategy) -> Self {
    let odict::Sense { pos, definitions } = sense;

    Self {
      pos: pos.to_string(),
      definitions: definitions
        .into_iter()
        .map(|d| match d {
          odict::DefinitionType::Definition(d) => Either::A(Definition::from(d, mds)),
          odict::DefinitionType::Group(g) => Either::B(Group::from(g, mds)),
        })
        .collect(),
    }
  }
}
