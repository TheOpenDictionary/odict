use serde::Serialize;
use wasm_bindgen::prelude::*;

use super::{definition::Definition, group::Group};

#[wasm_bindgen]
#[derive(Serialize)]
pub struct Sense {
  pos: String,
  definitions: Vec<JsValue>,
}

#[wasm_bindgen]
impl Sense {
  #[wasm_bindgen(getter)]
  pub fn pos(&self) -> String {
    self.pos.clone()
  }

  #[wasm_bindgen(getter)]
  pub fn definitions(&self) -> JsValue {
    serde_wasm_bindgen::to_value(&self.definitions).unwrap()
  }
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
