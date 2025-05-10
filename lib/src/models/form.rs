use crate::serializable;

use super::EntryRef;
use super::FormKind;

serializable! {
  #[serde(rename = "form")]
  pub struct Form {
    #[serde(rename = "@kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<FormKind>,

    #[serde(rename = "@term")]
    pub term: EntryRef,

    #[serde(default, rename = "tag")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
  }
}
