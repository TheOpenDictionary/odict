use internal::ToEnumWrapper;

use super::enums::EnumWrapper;

#[napi(object)]
#[derive(Clone)]
pub struct Form {
  pub term: String,
  pub kind: Option<EnumWrapper>,
  pub tags: Vec<String>,
}

impl From<odict::Form> for Form {
  fn from(form: odict::Form) -> Self {
    Self {
      term: form.term.to_string(),
      kind: form.kind.map(|k| k.to_enum_wrapper().into()),
      tags: form.tags,
    }
  }
}
