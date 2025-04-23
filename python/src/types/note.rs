use pyo3::prelude::*;
use structural_convert::StructuralConvert;

use super::Example;

#[pyclass]
#[derive(Debug, Clone, StructuralConvert)]
#[convert(from(odict::Note))]
pub struct Note {
    #[pyo3(get)]
    pub id: Option<String>,
    #[pyo3(get)]
    pub value: String,
    #[pyo3(get)]
    pub examples: Vec<Example>,
}
