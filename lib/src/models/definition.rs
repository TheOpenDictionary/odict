use crate::serializable;

use super::{example::Example, note::Note};

serializable! {
  pub struct Definition {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: String,

    #[serde(default, rename="example")]
    pub examples: Vec<Example>,

    #[serde(default, rename="note")]
    pub notes: Vec<Note>,
  }
}
