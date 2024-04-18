#![deny(clippy::all)]

use odict::{DictionaryFile, DictionaryReader};

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct DictionaryOptions {
  pub default_split_threshold: Option<u32>,
}

impl Default for DictionaryOptions {
  fn default() -> Self {
    DictionaryOptions {
      default_split_threshold: None,
    }
  }
}

#[napi]
struct Dictionary {
  pub path: String,
  options: Option<DictionaryOptions>,
  file: DictionaryFile,
}

#[napi]
impl Dictionary {
  #[napi(constructor)]
  pub fn new(path: String, options: Option<DictionaryOptions>) -> napi::Result<Self> {
    let reader = DictionaryReader::default();

    let file = reader
      .read_from_path(&path)
      .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e))?;

    Ok(Dictionary {
      path,
      options,
      file,
    })
  }
}
