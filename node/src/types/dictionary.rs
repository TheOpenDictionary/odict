use merge::Merge;

use super::SplitOptions;

use super::{IndexOptions, SearchOptions};

#[napi(object)]
#[derive(PartialEq, Merge, Clone, Eq)]
pub struct DictionaryOptions {
  #[merge(strategy = merge::option::recurse)]
  pub split: Option<SplitOptions>,
  #[merge(strategy = merge::option::recurse)]
  pub index: Option<IndexOptions>,
  #[merge(strategy = merge::option::recurse)]
  pub search: Option<SearchOptions>,
}

impl Default for DictionaryOptions {
  fn default() -> Self {
    DictionaryOptions {
      split: Some(SplitOptions::default()),
      index: Some(IndexOptions::default()),
      search: Some(SearchOptions::default()),
    }
  }
}
