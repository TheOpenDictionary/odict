use pyo3::prelude::*;
use structural_convert::StructuralConvert;

use super::{note::Note, Example};

/// A single definition of a word sense.
///
/// Contains the definition text along with optional examples and notes.
#[pyclass]
#[derive(Debug, Clone, StructuralConvert)]
#[convert(from(odict::schema::Definition))]
pub struct Definition {
    /// Optional identifier for this definition.
    #[pyo3(get)]
    pub id: Option<String>,
    /// The definition text.
    #[pyo3(get)]
    pub value: String,
    /// Usage examples illustrating this definition.
    #[pyo3(get)]
    pub examples: Vec<Example>,
    /// Additional notes about this definition.
    #[pyo3(get)]
    pub notes: Vec<Note>,
}
