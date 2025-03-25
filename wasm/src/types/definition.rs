use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::{note::Note, Example};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Definition {
  pub id: Option<String>,
  pub value: String,
  pub examples: Vec<Example>,
  pub notes: Vec<Note>,
}

impl Definition {
  pub fn from(definition: odict::Definition) -> Result<Self, JsValue> {
    let odict::Definition {
      id,
      value,
      examples,
      notes,
    } = definition;

    Ok(Self {
      id,
      value,
      examples: examples
        .into_iter()
        .map(|e| Example::from(e).unwrap())
        .collect::<Vec<Example>>(),
      notes: notes.into_iter().map(|n| Note::from(n).unwrap()).collect(),
    })
  }
}
