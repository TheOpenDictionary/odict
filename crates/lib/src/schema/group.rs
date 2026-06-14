use crate::serializable;

use super::definition::Definition;

serializable! {
  #[derive(Default)]
  #[serde(rename = "group")]
  pub struct Group {
    #[serde(rename = "@id")]
    #[rkyv(with = rkyv::with::Map<crate::intern::Intern>)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "@description")]
    #[rkyv(with = crate::intern::Intern)]
    pub description: String,

    #[serde(default, rename = "definition")]
    pub definitions: Vec<Definition>,
  }
}
