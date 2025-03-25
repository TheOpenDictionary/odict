use merge::Merge;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
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

impl From<IndexOptions> for odict::search::IndexOptions {
  fn from(opts: IndexOptions) -> Self {
    let mut s = odict::search::IndexOptions::default();

    if let Some(memory) = opts.memory {
      s = s.memory(memory as usize);
    }

    if let Some(directory) = opts.directory {
      s = s.dir(&directory);
    }

    if let Some(overwrite) = opts.overwrite {
      s = s.overwrite(overwrite);
    }

    s
  }
}
