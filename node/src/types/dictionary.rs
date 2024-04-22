use merge::Merge;

#[napi(object)]
#[derive(PartialEq, Merge, Clone, Eq)]
pub struct DictionaryOptions {
  pub default_split_threshold: Option<u32>,
}

impl Default for DictionaryOptions {
  fn default() -> Self {
    DictionaryOptions {
      default_split_threshold: None,
    }
  }
}
