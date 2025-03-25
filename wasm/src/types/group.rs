use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::definition::Definition;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Group {
  #[wasm_bindgen(skip)]
  pub id: Option<String>,
  #[wasm_bindgen(skip)]
  pub description: String,
  #[wasm_bindgen(skip)]
  pub definitions: Vec<Definition>,
}

#[wasm_bindgen]
impl Group {
  #[wasm_bindgen(getter)]
  pub fn get_id(&self) -> Option<String> {
    self.id.clone()
  }

  #[wasm_bindgen(getter)]
  pub fn get_description(&self) -> String {
    self.description.clone()
  }

  #[wasm_bindgen(getter)]
  pub fn get_definitions(&self) -> JsValue {
    serde_wasm_bindgen::to_value(&self.definitions).unwrap()
  }
}

impl Group {
  pub fn from(group: odict::Group) -> Result<Self, JsValue> {
    let odict::Group {
      id,
      description,
      definitions,
    } = group;

    Ok(Self {
      id,
      description,
      definitions: definitions
        .into_iter()
        .map(|d| Definition::from(d))
        .collect::<Result<Vec<Definition>, _>>()?,
    })
  }
}
