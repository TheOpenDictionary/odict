use merge::Merge;

#[napi(object)]
#[derive(PartialEq, Merge, Clone, Eq)]
pub struct SearchOptions {
  pub directory: Option<String>,
  pub threshold: Option<u32>,
  pub autoindex: Option<bool>,
  pub limit: Option<u32>,
}

impl Default for SearchOptions {
  fn default() -> Self {
    SearchOptions {
      threshold: None,
      directory: None,
      autoindex: None,
      limit: None,
    }
  }
}

#[cfg(feature = "search")]
impl From<odict::search::SearchOptions> for SearchOptions {
  fn from(opts: odict::search::SearchOptions) -> Self {
    SearchOptions {
      threshold: Some(opts.threshold),
      directory: Some(opts.dir.to_string_lossy().to_string()),
      autoindex: Some(opts.autoindex),
      limit: opts.limit,
    }
  }
}
