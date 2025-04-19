use napi::bindgen_prelude::*;

use super::form_kind::FormKind;

#[napi(object)]
#[derive(Clone)]
pub struct Form {
  pub term: String,
  pub kind: Option<FormKind>,
}

impl From<odict::Form> for Form {
  fn from(form: odict::Form) -> Self {
    Self {
      term: form.term.0,
      kind: form.kind.map(FormKind::from),
    }
  }
}
