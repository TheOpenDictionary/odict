use crate::types::{Pronunciation, Translation};

use pyo3::prelude::*;
use structural_convert::StructuralConvert;

/// A usage example illustrating a definition.
///
/// Examples can optionally include translations and pronunciations.
#[pyclass]
#[derive(Debug, Clone, StructuralConvert)]
#[convert(from(odict::schema::Example))]
pub struct Example {
    /// The example text.
    #[pyo3(get)]
    pub value: String,

    /// Translations of this example into other languages.
    #[pyo3(get)]
    pub translations: Vec<Translation>,

    /// Pronunciations for this example.
    #[pyo3(get)]
    pub pronunciations: Vec<Pronunciation>,
}

#[pymethods]
impl Example {
    #[new]
    pub fn new(value: String) -> Self {
        Self {
            value,
            translations: Vec::new(),
            pronunciations: Vec::new(),
        }
    }
}
