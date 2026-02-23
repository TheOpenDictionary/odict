use internal::ToEnumWrapper;
use pyo3::prelude::*;

use super::enums::EnumWrapper;

/// An inflected or alternate form of a word.
///
/// Forms represent morphological variants such as plurals, conjugations,
/// or other inflections.
#[pyclass]
#[derive(Clone, Debug)]
pub struct Form {
    /// The inflected form text.
    #[pyo3(get)]
    pub term: String,

    /// The kind of form (e.g. plural, past tense), or `None`.
    #[pyo3(get, set)]
    pub kind: Option<EnumWrapper>,

    /// Tags for categorizing this form.
    #[pyo3(get)]
    pub tags: Vec<String>,
}

impl From<odict::schema::Form> for Form {
    fn from(form: odict::schema::Form) -> Self {
        Self {
            term: form.term.to_string(),
            kind: form.kind.map(|k| k.to_enum_wrapper().into()),
            tags: form.tags,
        }
    }
}
