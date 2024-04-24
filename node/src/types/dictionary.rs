use merge::Merge;

use super::{IndexOptions, SplitOptions};

#[napi(object)]
#[derive(PartialEq, Merge, Clone, Eq)]
pub struct DictionaryOptions {
  pub split: Option<SplitOptions>,
  pub index: Option<IndexOptions>,
}

impl Default for DictionaryOptions {
  fn default() -> Self {
    DictionaryOptions {
      split: Some(SplitOptions::default()),
      index: Some(IndexOptions::default()),
    }
  }
}
