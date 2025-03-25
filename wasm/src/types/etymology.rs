use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use super::sense::Sense;

#[wasm_bindgen]
pub struct Etymology {
  pub id: Option<String>,
  pub pronunciation: Option<String>,
  pub description: Option<String>,
  pub senses: HashMap<String, Sense>,
}

impl Etymology {
  pub fn from(etymology: odict::Etymology) -> Result<Self, JsValue> {
    let odict::Etymology {
      id,
      pronunciation,
      description,
      senses,
    } = etymology;

    Ok(Self {
      id,
      pronunciation,
      description,
      senses: senses
        .into_iter()
        .map(|(k, v)| -> Result<(String, Sense), _> {
          let sense = Sense::from(v)?;
          Ok((k.to_string(), sense))
        })
        .collect::<Result<HashMap<String, Sense>, _>>()?,
    })
  }
}
