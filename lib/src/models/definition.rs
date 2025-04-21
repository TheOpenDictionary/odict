use crate::serializable;

use super::{
    example::{unwrap_examples, wrap_examples, Example},
    note::{unwrap_notes, wrap_notes, Note},
};

serializable! {
  pub struct Definition {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "@value")]
    pub value: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "unwrap_examples")]
    #[serde(serialize_with = "wrap_examples")]
    pub examples: Vec<Example>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "unwrap_notes")]
    #[serde(serialize_with = "wrap_notes")]
    pub notes: Vec<Note>,
  }
}
