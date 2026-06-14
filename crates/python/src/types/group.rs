use pyo3::prelude::*;
use structural_convert::StructuralConvert;

use super::definition::Definition;

/// A named group of related definitions.
///
/// Groups allow organizing multiple definitions under a shared description,
/// such as grouping definitions by semantic domain.
#[pyclass]
#[derive(Debug, Clone, StructuralConvert)]
#[convert(from(odict::schema::Group))]
pub struct Group {
    /// Optional identifier for this group.
    #[pyo3(get)]
    pub id: Option<String>,
    /// A description of what this group of definitions has in common.
    #[pyo3(get)]
    pub description: String,
    /// The definitions within this group.
    #[pyo3(get)]
    pub definitions: Vec<Definition>,
}
