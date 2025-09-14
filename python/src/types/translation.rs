use pyo3::prelude::*;
use structural_convert::StructuralConvert;

#[pyclass]
#[derive(Debug, Clone, StructuralConvert)]
#[convert(from(odict::schema::Translation))]
pub struct Translation {
    #[pyo3(get)]
    pub lang: String,

    #[pyo3(get)]
    pub value: String,
}

#[pymethods]
impl Translation {
    #[new]
    pub fn new(lang: String, value: String) -> Self {
        Self { lang, value }
    }
}
