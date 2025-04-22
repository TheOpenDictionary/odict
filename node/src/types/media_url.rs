use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct MediaURL {
  pub src: String,
  pub mime_type: Option<String>,
  pub description: Option<String>,
}

impl From<odict::MediaURL> for MediaURL {
  fn from(url: odict::MediaURL) -> Self {
    let odict::MediaURL {
      src,
      mime_type,
      description,
    } = url;
    Self {
      src,
      mime_type,
      description,
    }
  }
}
