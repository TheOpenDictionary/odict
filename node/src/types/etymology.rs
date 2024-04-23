use std::collections::HashMap;

use super::sense::Sense;

#[napi(object)]
pub struct Etymology {
  pub id: Option<String>,
  pub pronunciation: Option<String>,
  pub description: Option<String>,
  pub senses: HashMap<String, Sense>,
}

impl Etymology {
  pub fn from(etymology: odict::Etymology, mds: &odict::MarkdownStrategy) -> Self {
    let odict::Etymology {
      id,
      pronunciation,
      description,
      senses,
    } = etymology;

    Self {
      id,
      pronunciation,
      description: description.map(|d| d.parse(mds)),
      senses: senses
        .into_iter()
        .map(|(k, v)| (k, SenseJSON::from(v, mds)))
        .collect(),
    }
  }
}
