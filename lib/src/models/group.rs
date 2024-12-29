use crate::serializable;

use super::definition::Definition;

serializable! {
  pub struct Group {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "@description")]
    pub description: String,

    #[serde(default, rename = "definition")]
    pub definitions: Vec<Definition>,
  }
}
