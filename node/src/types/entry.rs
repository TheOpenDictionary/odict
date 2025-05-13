use napi::bindgen_prelude::*;
use structural_convert::StructuralConvert;

use crate::utils::cast_error;

use super::etymology::Etymology;
use super::media_url::MediaURL;

#[napi(object)]
#[derive(StructuralConvert)]
#[convert(from(odict::Entry))]
pub struct Entry {
  pub term: String,
  pub rank: Option<u32>,
  pub see_also: Option<String>,
  pub etymologies: Vec<Etymology>,
  pub media: Vec<MediaURL>,
}

pub fn from_archive(entry: &odict::ArchivedEntry) -> Result<Entry> {
  entry.to_entry().map_err(cast_error).map(|e| e.into())
}

impl TryFrom<&odict::Entry> for Entry {
  type Error = napi::Error;

  fn try_from(entry: &odict::Entry) -> Result<Self> {
    Ok(entry.clone().into())
  }
}

// Add this helper function to handle both Entry and ArchivedEntry types
pub fn from_entry(entry: &odict::Entry) -> Result<Entry> {
  Ok(entry.clone().into())
}
