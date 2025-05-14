use internal::ToEnumWrapper;
use pyo3::prelude::*;

use super::enums::EnumWrapper;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Form {
    #[pyo3(get)]
    pub term: String,

    #[pyo3(get, set)]
    pub kind: Option<EnumWrapper>,

    #[pyo3(get)]
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
