use pyo3::prelude::*;
use structural_convert::StructuralConvert;

use super::definition::Definition;

#[pyclass]
#[derive(Debug, Clone, StructuralConvert)]
#[convert(from(odict::schema::Group))]
pub struct Group {
    #[pyo3(get)]
    pub id: Option<String>,
    #[pyo3(get)]
    pub description: String,
    #[pyo3(get)]
    pub definitions: Vec<Definition>,
}
