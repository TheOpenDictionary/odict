use napi::bindgen_prelude::*;
use napi_derive::napi;
use structural_convert::StructuralConvert;

#[napi(object)]
#[derive(StructuralConvert)]
#[convert(from(odict::MediaURL))]
pub struct MediaURL {
  pub src: String,
  pub mime_type: Option<String>,
  pub description: Option<String>,
}
