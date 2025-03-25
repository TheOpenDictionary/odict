use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::sense::Sense;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Etymology {
  pub id: Option<String>,
  pub pronunciation: Option<String>,
  pub description: Option<String>,
  #[wasm_bindgen(skip)]
  pub senses: HashMap<String, Sense>,
}

#[wasm_bindgen]
impl Etymology {
  #[wasm_bindgen(getter)]
  pub fn get_senses(&self) -> JsValue {
    serde_wasm_bindgen::to_value(&self.senses).unwrap()
  }
}

impl Etymology {
  pub fn from(etymology: odict::Etymology) -> Result<Self, JsValue> {
    let odict::Etymology {
      id,
      pronunciation,
      description,
      senses,
    } = etymology;

    let senses_map = senses
        .into_iter()
        .map(|(k, v)| -> Result<(String, Sense), JsValue> {
          let sense = Sense::from(v)?;
          Ok((k.to_string(), sense))
        })
        .collect::<Result<HashMap<String, Sense>, JsValue>>()?;

    Ok(Self {
      id,
      pronunciation,
      description,
      senses: senses_map,
    })
  }
}
