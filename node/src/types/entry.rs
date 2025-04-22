use napi::bindgen_prelude::*;

use crate::utils::cast_error;

use super::etymology::Etymology;
use super::form::Form;
use super::pronunciation::Pronunciation;
use super::translation::Translation;

#[napi(object)]
pub struct Entry {
  pub term: String,
  pub see_also: Option<String>,
  pub etymologies: Vec<Etymology>,
  pub forms: Vec<Form>,
  pub translations: Vec<Translation>,
  pub pronunciations: Vec<Pronunciation>,
}

impl Entry {
  pub fn from_entry(entry: odict::Entry) -> Result<Self> {
    let odict::Entry {
      term,
      see_also,
      etymologies,
      forms,
      translations,
      pronunciations,
    } = entry;

    Ok(Self {
      term,
      see_also: see_also.map(|s| s.0),
      etymologies: etymologies
        .into_iter()
        .map(|e| Etymology::from(e))
        .collect::<Result<Vec<Etymology>, _>>()?,
      forms: forms.into_iter().map(Form::from).collect(),
      translations: translations
        .into_iter()
        .map(Translation::from)
        .collect::<Result<Vec<Translation>, _>>()?,
      pronunciations: pronunciations
        .into_iter()
        .map(|p| Pronunciation::from(p))
        .collect::<Result<Vec<Pronunciation>, _>>()?,
    })
  }

  pub fn from_archive(entry: &odict::ArchivedEntry) -> Result<Self> {
    Entry::from_entry(entry.to_entry().map_err(cast_error)?)
  }
}
