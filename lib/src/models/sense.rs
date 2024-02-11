use crate::serializable;

use super::{pos::PartOfSpeech, Definition, Group};

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

    #[serde(default, rename = "$value")]
    pub definitions: Vec<DefinitionType>,
  }
}
