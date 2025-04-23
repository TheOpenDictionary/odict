use pyo3::prelude::*;
use structural_convert::StructuralConvert;

use crate::utils::cast_error;

use super::etymology::Etymology;
use super::media_url::MediaURL;

#[pyclass]
#[derive(Debug, Clone, StructuralConvert)]
#[convert(from(odict::Entry))]
pub struct Entry {
    #[pyo3(get)]
    pub term: String,
    #[pyo3(get)]
    pub see_also: Option<String>,
    #[pyo3(get)]
    pub etymologies: Vec<Etymology>,
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
    pub fn from_archive(entry: &odict::ArchivedEntry) -> PyResult<Self> {
        Ok(Self::from(entry.to_entry().map_err(cast_error)?))
    }
}
