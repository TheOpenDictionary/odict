use wasm_bindgen::prelude::*;

use super::{definition::Definition, either::Either, group::Group};

#[wasm_bindgen]
pub struct Sense {
  pub pos: String,
  pub definitions: Vec<JsValue>,
}

impl Sense {
  pub fn from(sense: odict::Sense) -> Result<Self, JsValue> {
    let odict::Sense { pos, definitions } = sense;

    let defs = definitions
        .into_iter()
        .map(|d| -> Result<JsValue, JsValue> {
          match d {
            odict::DefinitionType::Definition(d) => {
              let def = Definition::from(d)?;
              Ok(serde_wasm_bindgen::to_value(&def).unwrap())
            },
            odict::DefinitionType::Group(g) => {
              let group = Group::from(g)?;
              Ok(serde_wasm_bindgen::to_value(&group).unwrap())
            },
          }
        })
        .collect::<Result<Vec<JsValue>, _>>()?;

    Ok(Self {
      pos: pos.to_string(),
      definitions: defs,
    })
  }
}
