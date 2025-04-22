use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Translation {
    #[pyo3(get)]
    pub lang: String,

    #[pyo3(get)]
    pub value: String,
}

impl Translation {
    pub fn from(translation: odict::Translation) -> PyResult<Self> {
        let odict::Translation { lang, value } = translation;
        Ok(Self { lang, value })
    }
}

#[pymethods]
impl Translation {
    #[new]
    pub fn new(lang: String, value: String) -> Self {
        Self { lang, value }
    }
}
