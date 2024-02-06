use crate::serializable;

use super::{pos::PartOfSpeech, Definition, Group};

serializable! {
  pub struct Sense {
    #[serde(rename = "@pos", default)]
    pub pos: PartOfSpeech,

    #[serde(rename = "group", default)]
    pub groups: Vec<Group>,

    #[serde(rename = "definition", default)]
    pub definitions: Vec<Definition>,
  }
}
