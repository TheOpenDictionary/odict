use crate::{serializable, MDString};

use super::example::Example;

serializable! {
  pub struct Note {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: MDString,

    #[serde(default, rename = "example")]
    pub examples: Vec<Example>,
  }
}
