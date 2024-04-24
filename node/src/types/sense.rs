use napi::bindgen_prelude::*;

use super::{definition::Definition, group::Group};

#[napi(object)]
pub struct Sense {
  pub pos: String,
  pub definitions: Vec<Either<Definition, Group>>,
}

impl Sense {
  pub fn from(env: napi::Env, sense: odict::Sense) -> Result<Self> {
    let odict::Sense { pos, definitions } = sense;

    Ok(Self {
      pos: pos.to_string(),
      definitions: definitions
        .into_iter()
        .map(|d| -> Result<Either<Definition, Group>> {
          match d {
            odict::DefinitionType::Definition(d) => Ok(Either::A(Definition::from(env, d)?)),
            odict::DefinitionType::Group(g) => Ok(Either::B(Group::from(env, g)?)),
          }
        })
        .collect::<Result<Vec<Either<Definition, Group>>, _>>()?,
    })
  }
}
