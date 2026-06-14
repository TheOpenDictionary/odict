use pyo3::prelude::*;
use structural_convert::StructuralConvert;

/// A translation of a word, definition, or example into another language.
#[pyclass]
#[derive(Debug, Clone, StructuralConvert)]
#[convert(from(odict::schema::Translation))]
pub struct Translation {
    /// The BCP-47 language code (e.g. `"fra"`, `"deu"`).
    #[pyo3(get)]
    pub lang: String,

    /// The translated text.
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
