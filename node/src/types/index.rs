use merge::Merge;

#[napi(object)]
#[derive(PartialEq, Debug, Merge, Clone, Eq)]
pub struct IndexOptions {
  #[merge(strategy = merge::option::overwrite_none)]
  pub directory: Option<String>,
  #[merge(strategy = merge::option::overwrite_none)]
  pub memory: Option<u32>,
  #[merge(strategy = merge::option::overwrite_none)]
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
impl From<IndexOptions> for odict::search::IndexOptions {
  fn from(opts: IndexOptions) -> Self {
    let mut options = odict::search::IndexOptions::default();

    if let Some(dir) = opts.directory {
      options = options.dir(dir.as_str());
    }

    if let Some(memory) = opts.memory {
      options = options.memory(memory as usize);
    }

    if let Some(overwrite) = opts.overwrite {
      options = options.overwrite(overwrite);
    }

    options
  }
}
