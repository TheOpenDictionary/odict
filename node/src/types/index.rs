use merge::Merge;

#[napi(object)]
#[derive(PartialEq, Merge, Clone, Eq)]
pub struct IndexOptions {
  pub directory: Option<String>,
  pub memory: Option<u32>,
  pub overwrite: Option<bool>,
}

impl Default for IndexOptions {
  fn default() -> Self {
    IndexOptions {
      overwrite: None,
      directory: None,
      memory: None,
    }
  }
}

#[cfg(feature = "search")]
impl From<odict::search::IndexOptions> for IndexOptions {
  fn from(opts: odict::search::IndexOptions) -> Self {
    IndexOptions {
      directory: Some(opts.dir.to_string_lossy().to_string()),
      memory: Some(opts.memory.try_into().unwrap()),
      overwrite: Some(opts.overwrite),
    }
  }
}
