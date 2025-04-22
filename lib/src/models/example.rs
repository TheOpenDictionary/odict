use crate::serializable;
use crate::Translation;

serializable! {
pub struct Example {
    #[serde(rename = "@value")]
    pub value: String,

    #[serde(default, rename = "translation")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translations: Vec<Translation>,
  }
}
