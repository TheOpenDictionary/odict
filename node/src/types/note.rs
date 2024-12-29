use napi::bindgen_prelude::*;

use super::Example;

#[napi(object)]
pub struct Note {
  pub id: Option<String>,
  pub value: String,
  pub examples: Vec<Example>,
}

impl Note {
  pub fn from(note: odict::Note) -> Result<Self> {
    let odict::Note {
      id,
      value,
      examples,
    } = note;

    Ok(Self {
      id,
      value,
      examples: examples
        .into_iter()
        .map(|e| Example::from(e))
        .collect::<Result<Vec<Example>, _>>()?,
    })
  }
}
