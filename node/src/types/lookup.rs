use merge::Merge;
use napi::bindgen_prelude::*;

use super::Entry;

#[napi(object)]
#[derive(Merge)]
pub struct LookupOptions {
  pub split: Option<u32>,
  pub follow: Option<bool>,
}

impl Default for LookupOptions {
  fn default() -> Self {
    LookupOptions {
      split: None,
      follow: None,
    }
  }
}

impl From<LookupOptions> for odict::lookup::LookupOptions {
  fn from(opts: LookupOptions) -> Self {
    let mut s = odict::lookup::LookupOptions::default();

    if let Some(split) = opts.split {
      s = s.strategy(odict::lookup::LookupStrategy::Split(split.try_into().unwrap()));
    }

    if let Some(follow) = opts.follow {
      s = s.follow(follow);
    }

    s
  }
}

#[napi(object)]
pub struct LookupResult {
  pub entry: Entry,
  pub directed_from: Option<Entry>,
}
