use crate::serializable;
use crate::{Pronunciation, Translation};

serializable! {
  #[derive(Default)]
  #[serde(rename = "example")]
  pub struct Example {
    #[serde(rename = "@value")]
    pub value: String,

    #[serde(default, rename = "translation")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translations: Vec<Translation>,

    #[serde(default, rename = "pronunciation")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pronunciations: Vec<Pronunciation>,
  }
}
