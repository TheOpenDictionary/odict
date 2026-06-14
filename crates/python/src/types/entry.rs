use pyo3::prelude::*;
use structural_convert::StructuralConvert;

use crate::utils::cast_error;

use super::etymology::Etymology;
use super::media_url::MediaURL;

/// A dictionary entry representing a single headword and its associated data.
///
/// Each entry contains the term itself, optional ranking metadata,
/// cross-reference information, etymologies, and media attachments.
#[pyclass]
#[derive(Debug, Clone, StructuralConvert)]
#[convert(from(odict::schema::Entry))]
pub struct Entry {
    /// The headword for this entry.
    #[pyo3(get)]
    pub term: String,
    /// Optional frequency rank for ordering entries.
    #[pyo3(get)]
    pub rank: Option<u32>,
    /// Cross-reference target term, if this entry redirects to another.
    #[pyo3(get)]
    pub see_also: Option<String>,
    /// The etymologies associated with this entry.
    #[pyo3(get)]
    pub etymologies: Vec<Etymology>,
    /// Media URLs (audio, images, etc.) associated with this entry.
    #[pyo3(get)]
    pub media: Vec<MediaURL>,
}

#[pymethods]
impl Entry {
    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    pub fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}

impl Entry {
    pub fn from_archive(entry: &odict::schema::ArchivedEntry) -> PyResult<Self> {
        Ok(Self::from(entry.deserialize().map_err(cast_error)?))
    }
}
