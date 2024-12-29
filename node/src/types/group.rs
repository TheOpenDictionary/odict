use napi::bindgen_prelude::*;

use super::definition::Definition;

#[napi(object)]
pub struct Group {
  pub id: Option<String>,
  pub description: String,
  pub definitions: Vec<Definition>,
}

impl Group {
  pub fn from(group: odict::Group) -> Result<Self> {
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
