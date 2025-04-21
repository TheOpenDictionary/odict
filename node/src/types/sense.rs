use napi::bindgen_prelude::*;

use super::{definition::Definition, group::Group};

#[napi(object)]
pub struct Sense {
  pub pos: String,
  pub lemma: Option<String>,
  pub definitions: Vec<Either<Definition, Group>>,
  pub tags: Vec<String>,
}

impl Sense {
  pub fn from(sense: odict::Sense) -> Result<Self> {
    let odict::Sense {
      pos,
      lemma,
      definitions,
      tags,
    } = sense;

    Ok(Self {
      pos: pos.to_string(),
      lemma: lemma.map(|l| l.0),
      definitions: definitions
        .into_iter()
        .map(|d| -> Result<Either<Definition, Group>> {
          match d {
            odict::DefinitionType::Definition(d) => Ok(Either::A(Definition::from(d)?)),
            odict::DefinitionType::Group(g) => Ok(Either::B(Group::from(g)?)),
          }
        })
        .collect::<Result<Vec<Either<Definition, Group>>, _>>()?,
      tags,
    })
  }
}
