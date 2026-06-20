use std::fmt;

use pyo3::prelude::*;

use super::pronunciation::Pronunciation;
use super::sense::Sense;

/// An etymology grouping for a dictionary entry.
///
/// Etymologies group together senses that share a common word origin.
/// Each etymology can have its own pronunciations and description.
#[pyclass]
#[derive(Clone)]
pub struct Etymology {
    /// Optional identifier for this etymology.
    #[pyo3(get)]
    pub id: Option<String>,
    /// Pronunciations associated with this etymology.
    #[pyo3(get)]
    pub pronunciations: Vec<Pronunciation>,
    /// Optional description of the word origin.
    #[pyo3(get)]
    pub description: Option<String>,
    /// The senses (meanings) under this etymology.
    #[pyo3(get)]
    pub senses: Vec<Sense>,
}

impl fmt::Debug for Etymology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Etymology")
            .field("id", &self.id)
            .field("pronunciations", &self.pronunciations)
            .field("description", &self.description)
            .field("senses", &self.senses)
            .finish()
    }
}

impl From<odict::schema::Etymology> for Etymology {
    fn from(ety: odict::schema::Etymology) -> Self {
        Self {
            id: ety.id,
            pronunciations: ety.pronunciations.into_iter().map(Into::into).collect(),
            description: ety.description,
            senses: ety.senses.into_iter().map(|s| s.into()).collect(),
        }
    }
}
