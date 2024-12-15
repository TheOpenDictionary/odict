use merge::Merge;
use napi::Either;
use odict::error::Error;

use crate::types::{DictionaryOptions, LookupQuery};

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

pub fn to_lookup_query(query: Either<LookupQuery, String>) -> odict::lookup::LookupQuery {
  match query {
    Either::A(wwf) => wwf.into(),
    Either::B(s) => s.into(),
  }
}
