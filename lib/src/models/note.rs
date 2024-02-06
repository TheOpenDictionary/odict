use crate::serializable;

use super::example::Example;

serializable! {
  pub struct Note {
    #[serde(rename = "@id")]
    id: Option<String>,

    // #[serde(rename = "@value")]
    // value: MDString,
    #[serde(default, rename = "example")]
    examples: Vec<Example>,
  }
}
