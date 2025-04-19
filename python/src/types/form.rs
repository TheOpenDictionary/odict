use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Form {
    #[pyo3(get)]
    pub term: String,

    #[pyo3(get)]
    pub kind: Option<String>,
}

#[pymethods]
impl Form {
    #[new]
    pub fn new(term: String, kind: Option<String>) -> Self {
        Self { term, kind }
    }
}

impl From<odict::Form> for Form {
    fn from(form: odict::Form) -> Self {
        Self {
            term: form.term.0,
            kind: form.kind.map(|t| t.to_string()),
        }
    }
}
