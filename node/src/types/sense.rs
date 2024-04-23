use napi::Either;

use super::{definition::Definition, group::Group};

#[napi(object)]
pub struct Sense {
  pub pos: String,
  pub definitions: Vec<Either<Group, Either<Definition, Group>>>,
}

impl Sense {
  fn from(sense: odict::Sense, mds: odict::MarkdownStrategy) -> Self {
    let odict::Sense { pos, definitions } = sense;

    Self {
      pos,
      definitions: definitions.into_iter().map(|d| Either::from(d)).collect(),
    }
  }
}
