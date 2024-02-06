use crate::serializable;

use super::{definition::Definition, mdstring::MDString};

serializable! {
  pub struct Group {
    #[serde(rename = "@id")]
    id: Option<String>,

    #[serde(rename = "@description")]
    description: Option<MDString>,

    #[serde(default, rename = "definition")]
    definitions: Vec<Definition>,
  }
}
