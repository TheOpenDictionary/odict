use pyo3::prelude::*;

use crate::utils::cast_error;

use super::etymology::Etymology;

#[pyclass]
#[derive(Debug)]
pub struct Entry {
    pub term: String,
    pub see_also: Option<String>,
    pub etymologies: Vec<Etymology>,
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
    pub fn from_entry(entry: odict::Entry) -> PyResult<Self> {
        let odict::Entry {
            term,
            see_also,
            etymologies,
        } = entry;

        Ok(Self {
            term,
            see_also,
            etymologies: etymologies
                .into_iter()
                .map(|e| Etymology::from(e))
                .collect::<Result<Vec<Etymology>, _>>()?,
        })
    }

    pub fn from_archive(entry: &odict::ArchivedEntry) -> PyResult<Self> {
        Entry::from_entry(entry.to_entry().map_err(cast_error)?)
    }
}
