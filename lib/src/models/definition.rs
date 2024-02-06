use crate::serializable;

use super::{example::Example, note::Note, MDString};

serializable! {
  pub struct Definition {
    #[serde(rename = "@id")]
    id: Option<String>,

    #[serde(rename = "@value")]
    value: MDString,

    #[serde(default)]
    examples: Vec<Example>,

    #[serde(default)]
    notes: Vec<Note>,
  }
}
