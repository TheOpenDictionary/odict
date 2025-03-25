use wasm_bindgen::prelude::*;

use super::{definition::Definition, group::Group};

#[wasm_bindgen]
pub struct Sense {
  pub pos: String,
  pub definitions: Vec<Either<Definition, Group>>,
}

impl Sense {
  pub fn from(sense: odict::Sense) -> Result<Self> {
    let odict::Sense { pos, definitions } = sense;

    Ok(Self {
      pos: pos.to_string(),
      definitions: definitions
        .into_iter()
        .map(|d| -> Result<Either<Definition, Group>> {
          match d {
            odict::DefinitionType::Definition(d) => Ok(Either::A(Definition::from(d)?)),
            odict::DefinitionType::Group(g) => Ok(Either::B(Group::from(g)?)),
          }
        })
        .collect::<Result<Vec<Either<Definition, Group>>, _>>()?,
    })
  }
}
