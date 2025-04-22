use merge::Merge;
use odict::lookup::LookupStrategy;

use super::Entry;

#[napi(object)]
#[derive(Merge)]
pub struct LookupOptions {
  pub split: Option<u32>,
  pub follow: Option<bool>,
  pub insensitive: Option<bool>,
}

impl Default for LookupOptions {
  fn default() -> Self {
    LookupOptions {
      split: None,
      follow: None,
      insensitive: None,
    }
  }
}

impl From<odict::lookup::LookupOptions> for LookupOptions {
  fn from(opts: odict::lookup::LookupOptions) -> Self {
    let mut s = LookupOptions::default();

    if let LookupStrategy::Split(split) = opts.strategy {
      s.split = Some(split.try_into().unwrap());
    }

    s.follow = Some(opts.follow);
    s.insensitive = Some(opts.insensitive);
  }
}

#[napi(object)]
#[derive(StructuralConvert)]
#[convert(from(odict::lookup::LookupResult))]
pub struct LookupResult {
  pub entry: Entry,
  pub directed_from: Option<Entry>,
}
