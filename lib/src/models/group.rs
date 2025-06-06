use rkyv::with::{AsBox, MapNiche};

use crate::serializable;

use super::definition::Definition;

serializable! {
  #[derive(Default)]
  #[serde(rename = "group")]
  pub struct Group {
    #[serde(rename = "@id")]
    #[rkyv(with = MapNiche<AsBox>)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "@description")]
    pub description: String,

    #[serde(default, rename = "definition")]
    pub definitions: Vec<Definition>,
  }
}
