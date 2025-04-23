use napi_derive::napi;
use structural_convert::StructuralConvert;

use super::media_url::MediaURL;
use super::pronunciation_kind::PronunciationKind;

#[napi(object)]
#[derive(StructuralConvert)]
#[convert(from(odict::Pronunciation))]
pub struct Pronunciation {
  pub kind: PronunciationKind,
  pub value: String,
  pub urls: Vec<MediaURL>,
}
