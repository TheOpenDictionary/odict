use napi::bindgen_prelude::*;

use super::{mdstring::MDString, note::Note, Example};

#[napi(object)]
pub struct Definition {
  pub id: Option<String>,
  pub value: ClassInstance<MDString>,
  pub examples: Vec<Example>,
  pub notes: Vec<Note>,
}

impl Definition {
  pub fn from(env: napi::Env, definition: odict::Definition) -> Result<Self> {
    let odict::Definition {
      id,
      value,
      examples,
      notes,
    } = definition;

    Ok(Self {
      id,
      value: MDString::from(value).into_instance(env)?,
      examples: examples
        .into_iter()
        .map(|e| Example::from(env, e).unwrap())
        .collect::<Vec<Example>>(),
      notes: notes
        .into_iter()
        .map(|n| Note::from(env, n).unwrap())
        .collect(),
    })
  }
}
