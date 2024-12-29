use crate::serializable;

use super::example::Example;

serializable! {
  pub struct Note {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: String,

    #[serde(default, rename = "example")]
    pub examples: Vec<Example>,
  }
}
