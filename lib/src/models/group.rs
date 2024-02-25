use crate::serializable;

use super::{definition::Definition, mdstring::MDString};

serializable! {
  pub struct Group {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "@description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<MDString>,

    #[serde(default, rename = "definition")]
    pub definitions: Vec<Definition>,
  }
}
