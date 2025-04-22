use either::Either;
use pyo3::prelude::*;
use structural_convert::StructuralConvert;

use super::{definition::Definition, group::Group};

#[pyclass]
#[derive(Debug, Clone, StructuralConvert)]
#[convert(from(odict::Sense))]
pub struct Sense {
    #[pyo3(get)]
    pub pos: String,
    #[pyo3(get)]
    pub lemma: Option<String>,
    #[pyo3(get)]
    pub definitions: Vec<Either<Definition, Group>>,
    #[pyo3(get)]
    pub tags: Vec<String>,
}
