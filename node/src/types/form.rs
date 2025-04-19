use napi::bindgen_prelude::*;

#[napi(object)]
#[derive(Clone)]
pub struct Form {
  pub term: String,
  pub kind: Option<String>,
}

impl From<odict::Form> for Form {
  fn from(form: odict::Form) -> Self {
    Self {
      term: form.term.0,
      kind: form.kind.map(|t| t.to_string()),
    }
  }
}
