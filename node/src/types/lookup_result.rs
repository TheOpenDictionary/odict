use napi::bindgen_prelude::*;

use super::Entry;

#[napi(object)]
pub struct LookupResult {
  pub entry: Entry,
  pub directed_from: Option<Entry>,
}
