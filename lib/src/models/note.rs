use crate::{serializable, MDString};

use super::example::Example;

serializable! {
  pub struct Note {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: MDString,

    #[serde(default, rename = "example")]
    pub examples: Vec<Example>,
  }
}
