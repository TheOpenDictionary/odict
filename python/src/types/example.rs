use crate::types::{Pronunciation, Translation};

use pyo3::prelude::*;
use structural_convert::StructuralConvert;

#[pyclass]
#[derive(Debug, Clone, StructuralConvert)]
#[convert(from(odict::Example))]
pub struct Example {
    #[pyo3(get)]
    pub value: String,

    #[pyo3(get)]
    pub translations: Vec<Translation>,

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
