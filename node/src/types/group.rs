use napi::bindgen_prelude::*;

use super::{definition::Definition, MDString};

#[napi(object)]
pub struct Group {
  pub id: Option<String>,
  pub description: ClassInstance<MDString>,
  pub definitions: Vec<Definition>,
}

impl Group {
  pub fn from(env: napi::Env, group: odict::Group) -> Result<Self> {
    let odict::Group {
      id,
      description,
      definitions,
    } = group;

    Ok(Self {
      id,
      description: MDString::from(description).into_instance(env)?,
      definitions: definitions
        .into_iter()
        .map(|d| Definition::from(env, d))
        .collect::<Result<Vec<Definition>, _>>()?,
    })
  }
}
