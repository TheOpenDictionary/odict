use rkyv::with::{AsBox, MapNiche};

use crate::serializable;

use super::EntryRef;
use super::FormKind;

serializable! {
  #[serde(rename = "form")]
  pub struct Form {
    #[serde(rename = "@kind")]
    #[rkyv(with = MapNiche<AsBox>)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<FormKind>,

    #[serde(rename = "@term")]
    // TODO: intern
    pub term: EntryRef,

    #[serde(default, rename = "tag")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
  }
}
