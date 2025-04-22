use napi::bindgen_prelude::*;
use structural_convert::StructuralConvert;

use crate::utils::cast_error;

use super::etymology::Etymology;
use super::form::Form;
use super::translation::Translation;

#[napi(object)]
#[derive(StructuralConvert)]
#[convert(from(odict::Entry))]
pub struct Entry {
  pub term: String,
  pub see_also: Option<String>,
  pub etymologies: Vec<Etymology>,
  pub forms: Vec<Form>,
  pub translations: Vec<Translation>,
}

impl TryFrom<&odict::ArchivedEntry> for Entry {
  fn try_from(entry: &odict::ArchivedEntry) -> Result<Self> {
    entry.to_entry().map_err(cast_error)?.into()
  }

  type Error = napi::Error;
}
