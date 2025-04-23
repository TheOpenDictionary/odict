use pyo3::prelude::*;
use structural_convert::StructuralConvert;

use super::form_kind::FormKind;

#[pyclass]
#[derive(Clone, Debug, StructuralConvert)]
#[convert(from(odict::Form))]
pub struct Form {
    #[pyo3(get)]
    pub term: String,

    #[pyo3(get, set)]
    pub kind: Option<FormKind>,

    #[pyo3(get)]
    pub tags: Vec<String>,
}

#[pymethods]
impl Form {
    #[new]
    pub fn new(term: String, kind: Option<FormKind>, tags: Option<Vec<String>>) -> Self {
        Self {
            term,
            kind,
            tags: tags.unwrap_or_default(),
        }
    }

    fn __str__(&self) -> String {
        if let Some(k) = &self.kind {
            format!("Form({}, kind={})", self.term, k.to_string())
        } else {
            format!("Form({})", self.term)
        }
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}
