use pyo3::prelude::*;
use structural_convert::StructuralConvert;

use super::Example;

/// An additional note attached to a definition.
///
/// Notes provide supplementary information such as usage guidance,
/// historical context, or grammatical remarks.
#[pyclass]
#[derive(Debug, Clone, StructuralConvert)]
#[convert(from(odict::schema::Note))]
pub struct Note {
    /// Optional identifier for this note.
    #[pyo3(get)]
    pub id: Option<String>,
    /// The note text.
    #[pyo3(get)]
    pub value: String,
    /// Examples associated with this note.
    #[pyo3(get)]
    pub examples: Vec<Example>,
}
