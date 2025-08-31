use pyo3::prelude::*;
use structural_convert::StructuralConvert;

use super::{note::Note, Example};

#[pyclass]
#[derive(Debug, Clone, StructuralConvert)]
#[convert(from(odict::schema::Definition))]
pub struct Definition {
    #[pyo3(get)]
    pub id: Option<String>,
    #[pyo3(get)]
    pub value: String,
    #[pyo3(get)]
    pub examples: Vec<Example>,
    #[pyo3(get)]
    pub notes: Vec<Note>,
}
