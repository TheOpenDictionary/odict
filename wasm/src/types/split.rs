use merge::Merge;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(PartialEq, Merge, Clone, Eq)]
pub struct SplitOptions {
  pub threshold: Option<u32>,
}

impl Default for SplitOptions {
  fn default() -> Self {
    SplitOptions { threshold: None }
  }
}

impl From<SplitOptions> for odict::split::SplitOptions {
  fn from(opts: SplitOptions) -> Self {
    let mut s = odict::split::SplitOptions::default();

    if let Some(threshold) = opts.threshold {
      s = s.threshold(threshold.try_into().unwrap());
    }

    s
  }
}
