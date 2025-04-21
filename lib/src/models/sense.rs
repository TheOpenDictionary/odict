use crate::serializable;

use super::{
    pos::PartOfSpeech,
    tags::{unwrap_tags, wrap_tags},
    Definition, EntryRef, Group,
};

serializable! {
  pub enum DefinitionType {
    #[serde(rename = "group")]
    Group(Group),

    #[serde(rename = "definition")]
    Definition(Definition),
  }
}

serializable! {
  pub struct Sense {
    #[serde(rename = "@pos", default)]
    pub pos: PartOfSpeech,

    #[serde(rename = "@lemma")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lemma: Option<EntryRef>,

    #[serde(default, rename = "$value")]
    pub definitions: Vec<DefinitionType>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "unwrap_tags")]
    #[serde(serialize_with = "wrap_tags")]
    pub tags: Vec<String>,
  }
}
