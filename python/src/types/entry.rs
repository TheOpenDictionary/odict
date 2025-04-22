use pyo3::prelude::*;

use crate::utils::cast_error;

use super::etymology::Etymology;
use super::form::Form;
use super::pronunciation::Pronunciation;
use super::translation::Translation;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Entry {
    #[pyo3(get)]
    pub term: String,
    #[pyo3(get)]
    pub see_also: Option<String>,
    #[pyo3(get)]
    pub etymologies: Vec<Etymology>,
    #[pyo3(get)]
    pub forms: Vec<Form>,
    #[pyo3(get)]
    pub translations: Vec<Translation>,
    #[pyo3(get)]
    pub pronunciations: Vec<Pronunciation>,
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

impl From<odict::Entry> for Entry {
    fn from(entry: odict::Entry) -> Self {
        let odict::Entry {
            term,
            see_also,
            etymologies,
            forms,
            translations,
            pronunciations,
        } = entry;

        Self {
            term,
            see_also: see_also.map(|s| s.0),
            etymologies: etymologies
                .into_iter()
                .map(|e| Etymology::from(e))
                .collect(),
            forms: forms.into_iter().map(Form::from).collect(),
            translations: translations
                .into_iter()
                .map(|t| Translation::from(t).unwrap())
                .collect(),
            pronunciations: pronunciations
                .into_iter()
                .map(Pronunciation::from)
                .collect(),
        }
    }
}
