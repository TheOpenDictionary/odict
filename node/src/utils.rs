use merge::Merge;
use odict::Error;

use crate::types::DictionaryOptions;

pub fn cast_error(e: Error) -> napi::Error {
  napi::Error::new(napi::Status::GenericFailure, format!("{}", e))
}

pub fn resolve_options(options: &Option<DictionaryOptions>) -> DictionaryOptions {
  match options {
    Some(opts) => {
      let mut out = opts.clone();
      out.merge(DictionaryOptions::default());
      return out;
    }
    None => DictionaryOptions::default(),
  }
}
