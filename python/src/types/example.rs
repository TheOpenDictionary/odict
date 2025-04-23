use crate::types::{Pronunciation, Translation};

use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Example {
    #[pyo3(get)]
    pub value: String,

    #[pyo3(get)]
    pub translations: Vec<Translation>,

    #[pyo3(get)]
    pub pronunciations: Vec<Pronunciation>,
}

impl From<odict::Example> for Example {
    fn from(example: odict::Example) -> Self {
        let odict::Example {
            value,
            translations,
            pronunciations,
        } = example;

        let mapped_translations = translations
            .into_iter()
            .map(|t| Translation::from(t).unwrap())
            .collect();

        let mapped_pronunciations = pronunciations
            .into_iter()
            .map(Pronunciation::from)
            .collect();

        Self {
            value: String::from(value),
            translations: mapped_translations,
            pronunciations: mapped_pronunciations,
        }
    }
}

#[pymethods]
impl Example {
    #[new]
    pub fn new(value: String) -> Self {
        Self {
            value,
            translations: Vec::new(),
            pronunciations: Vec::new(),
        }
    }
}
