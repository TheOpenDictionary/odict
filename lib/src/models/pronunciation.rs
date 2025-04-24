use super::{MediaURL, PronunciationKind};
use crate::serializable;

serializable! {
  pub struct Pronunciation {
    #[serde(rename = "@kind")]
    pub kind: PronunciationKind,

    #[serde(rename = "@value")]
    pub value: String,

    #[serde(default, rename = "url")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub media: Vec<MediaURL>,
  }
}
