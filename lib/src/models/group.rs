use crate::serializable;

use super::{definition::Definition, mdstring::MDString};

serializable! {
  pub struct Group {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@description")]
    pub description: MDString,

    #[serde(default, rename = "definition")]
    pub definitions: Vec<Definition>,
  }
}
