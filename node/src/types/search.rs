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
impl From<SearchOptions> for odict::search::SearchOptions {
  fn from(opts: SearchOptions) -> Self {
    let mut s = odict::search::SearchOptions::default();

    if let Some(threshold) = opts.threshold {
      s = s.threshold(threshold);
    }

    if let Some(directory) = opts.directory {
      s = s.dir(&directory);
    }

    if let Some(limit) = opts.limit {
      s = s.limit(limit.try_into().unwrap());
    }

    if let Some(autoindex) = opts.autoindex {
      s = s.autoindex(autoindex);
    }

    s
  }
}
