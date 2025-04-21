use crate::serializable;

use super::{pos::PartOfSpeech, Definition, Group, EntryRef};

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
  }
}
