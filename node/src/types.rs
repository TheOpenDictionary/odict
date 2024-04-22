use merge::Merge;
use odict::lookup;

#[napi]
pub enum MarkdownStrategy {
  Disable,
  HTML,
  Text,
}

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

#[napi(object)]
pub struct LookupOptions {
  pub split: Option<u32>,
  pub markdown_strategy: Option<MarkdownStrategy>,
  pub follow: Option<bool>,
}

impl Default for LookupOptions {
  fn default() -> Self {
    LookupOptions {
      split: None,
      markdown_strategy: None,
      follow: None,
    }
  }
}

#[napi(object)]
pub struct LookupQuery {
  pub term: String,
  pub fallback: String,
}

impl From<LookupQuery> for lookup::LookupQuery {
  fn from(q: LookupQuery) -> Self {
    lookup::LookupQuery {
      term: q.term,
      fallback: q.fallback,
    }
  }
}
