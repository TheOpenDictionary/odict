use merge::Merge;

#[napi(object)]
#[derive(Merge)]
pub struct LookupOptions {
  pub split: Option<u32>,
  #[napi(ts_type = "'disable' | 'html' | 'text'")]
  pub markdown_strategy: Option<String>,
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

impl From<LookupQuery> for odict::lookup::LookupQuery {
  fn from(q: LookupQuery) -> Self {
    odict::lookup::LookupQuery {
      term: q.term,
      fallback: q.fallback,
    }
  }
}
