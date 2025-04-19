use napi::bindgen_prelude::*;

#[napi]
pub enum FormKind {
  Conjugation,
  Inflection,
  Plural,
  Singular,
  Comparative,
  Superlative,
  Other,
}

impl From<odict::FormKind> for FormKind {
  fn from(kind: odict::FormKind) -> Self {
    match kind {
      odict::FormKind::Conjugation => FormKind::Conjugation,
      odict::FormKind::Inflection => FormKind::Inflection,
      odict::FormKind::Plural => FormKind::Plural,
      odict::FormKind::Singular => FormKind::Singular,
      odict::FormKind::Comparative => FormKind::Comparative,
      odict::FormKind::Superlative => FormKind::Superlative,
      odict::FormKind::Other => FormKind::Other,
    }
  }
}
