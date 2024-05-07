use napi::bindgen_prelude::*;

use crate::utils::cast_error;

use super::etymology::Etymology;

#[napi(object)]
pub struct Entry {
  pub term: String,
  pub see_also: Option<String>,
  pub etymologies: Vec<Etymology>,
}

impl Entry {
  pub fn from_entry(env: napi::Env, entry: odict::Entry) -> Result<Self> {
    let odict::Entry {
      term,
      see_also,
      etymologies,
    } = entry;

    Ok(Self {
      term,
      see_also,
      etymologies: etymologies
        .into_iter()
        .map(|e| Etymology::from(env, e))
        .collect::<Result<Vec<Etymology>, _>>()?,
    })
  }

  pub fn from_archive(env: napi::Env, entry: &odict::ArchivedEntry) -> Result<Self> {
    Entry::from_entry(env, entry.to_entry().map_err(cast_error)?)
  }
}
