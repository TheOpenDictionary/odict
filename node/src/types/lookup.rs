use merge::Merge;
use odict::ArchivedEntry;

use crate::utils::cast_error;

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

impl From<LookupOptions> for odict::lookup::LookupOptions {
  fn from(opts: LookupOptions) -> Self {
    let mut options = odict::lookup::LookupOptions::default();

    if let Some(split) = opts.split {
      options = options.strategy(odict::lookup::LookupStrategy::Split(split as usize));
    }

    if let Some(follow) = opts.follow {
      options = options.follow(follow);
    }

    if let Some(insensitive) = opts.insensitive {
      options = options.insensitive(insensitive);
    }

    options
  }
}

#[napi(object)]
pub struct LookupResult {
  pub entry: Entry,
  pub directed_from: Option<Entry>,
}

impl From<odict::lookup::LookupResult<odict::Entry>> for LookupResult {
  fn from(result: odict::lookup::LookupResult<odict::Entry>) -> Self {
    let entry = Entry::from(result.entry);
    let directed_from = result.directed_from.map(|s| Entry::from(s));

    Self {
      entry,
      directed_from,
    }
  }
}

impl TryFrom<&odict::lookup::LookupResult<&ArchivedEntry>> for LookupResult {
  type Error = napi::Error;

  fn try_from(result: &odict::lookup::LookupResult<&ArchivedEntry>) -> napi::Result<Self> {
    let entry = Entry::from(result.entry.to_entry().map_err(cast_error)?);

    let directed_from = match result.directed_from {
      Some(e) => Some(Entry::from(e.to_entry().map_err(cast_error)?)),
      None => None,
    };

    Ok(LookupResult {
      entry,
      directed_from,
    })
  }
}
