use internal::ToEnumWrapper;
use napi_derive::napi;

use super::enums::EnumWrapper;
use super::media_url::MediaURL;

#[napi(object)]
pub struct Pronunciation {
  pub kind: Option<EnumWrapper>,
  pub value: String,
  pub media: Vec<MediaURL>,
}

impl From<odict::Pronunciation> for Pronunciation {
  fn from(pronunciation: odict::Pronunciation) -> Self {
    Self {
      kind: pronunciation.kind.map(|k| k.to_enum_wrapper().into()),
      value: pronunciation.value,
      media: pronunciation
        .media
        .into_iter()
        .map(MediaURL::from)
        .collect(),
    }
  }
}
