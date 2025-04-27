use napi_derive::napi;
use structural_convert::StructuralConvert;

use super::media_url::MediaURL;

#[napi(object)]
#[derive(StructuralConvert)]
#[convert(from(odict::Pronunciation))]
pub struct Pronunciation {
  pub kind: Option<String>,
  pub value: String,
  pub urls: Vec<MediaURL>,
}
