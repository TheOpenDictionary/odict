use napi::bindgen_prelude::*;

use super::{mdstring::MDString, Example};

#[napi(object)]
pub struct Note {
  pub id: Option<String>,
  pub value: ClassInstance<MDString>,
  pub examples: Vec<Example>,
}

impl Note {
  pub fn from(env: napi::Env, note: odict::Note) -> Result<Self> {
    let odict::Note {
      id,
      value,
      examples,
    } = note;

    Ok(Self {
      id,
      value: MDString::from(value).into_instance(env)?,
      examples: examples
        .into_iter()
        .map(|e| Example::from(env, e))
        .collect::<Result<Vec<Example>, _>>()?,
    })
  }
}
