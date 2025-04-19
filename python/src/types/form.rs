use pyo3::prelude::*;

use super::form_kind::FormKind;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Form {
    #[pyo3(get)]
    pub term: String,

    #[pyo3(get, set)]
    pub kind: Option<FormKind>,
}

#[pymethods]
impl Form {
    #[new]
    pub fn new(term: String, kind: Option<FormKind>) -> Self {
        Self { term, kind }
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

impl From<odict::Form> for Form {
    fn from(form: odict::Form) -> Self {
        Self {
            term: form.term.0,
            kind: form.kind.map(FormKind::from),
        }
    }
}
