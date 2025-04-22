use napi::bindgen_prelude::*;
use napi_derive::napi;

use super::media_url::MediaURL;
use super::pronunciation_kind::PronunciationKind;

#[napi(object)]
pub struct Pronunciation {
  pub kind: PronunciationKind,
  pub value: String,
  pub urls: Vec<MediaURL>,
}

impl Pronunciation {
  pub fn from(pronunciation: odict::Pronunciation) -> Result<Self> {
    let odict::Pronunciation { kind, value, urls } = pronunciation;

    Ok(Self {
      kind: kind.into(),
      value,
      urls: urls.into_iter().map(MediaURL::from).collect(),
    })
  }
}
