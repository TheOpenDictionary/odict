use crate::models::form::Form;
use crate::{serializable, Translation};

use super::{pos::PartOfSpeech, Definition, EntryRef, Group};

serializable! {
  pub enum DefinitionType {
    #[serde(rename = "group")]
    Group(Group),

    #[serde(rename = "definition")]
    Definition(Definition),
  }
}

serializable! {
  #[serde(rename = "sense")]
  pub struct Sense {
    #[serde(rename = "@pos", default)]
    pub pos: PartOfSpeech,

    #[serde(rename = "@lemma")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lemma: Option<EntryRef>,

    #[serde(default, rename = "$value")]
    pub definitions: Vec<DefinitionType>,

    #[serde(default, rename = "tag")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    #[serde(default, rename = "translation")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translations: Vec<Translation>,

    #[serde(default, rename = "form")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub forms: Vec<Form>,
  }
}
