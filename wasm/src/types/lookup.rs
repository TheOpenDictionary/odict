use merge::Merge;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
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
      s = s.split(split.try_into().unwrap());
    }

    if let Some(follow) = opts.follow {
      s = s.follow(follow);
    }

    s
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
