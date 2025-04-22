use pyo3::prelude::*;
use std::fmt;

use super::media_url::MediaURL;
use super::pronunciation_kind::PronunciationKind;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Pronunciation {
    #[pyo3(get)]
    pub kind: PronunciationKind,

    #[pyo3(get)]
    pub value: String,

    #[pyo3(get)]
    pub urls: Vec<MediaURL>,
}

#[pymethods]
impl Pronunciation {
    #[new]
    pub fn new(kind: &str, value: String) -> Self {
        Self {
            kind: PronunciationKind::new(kind),
            value,
            urls: Vec::new(),
        }
    }

    pub fn add_url(&mut self, url: MediaURL) {
        self.urls.push(url);
    }

    fn __repr__(&self) -> String {
        format!(
            "Pronunciation(kind='{}', value='{}', urls={})",
            self.raw_kind,
            self.value,
            if self.urls.is_empty() {
                "[]".to_string()
            } else {
                format!(
                    "[{}]",
                    self.urls
                        .iter()
                        .map(|url| url.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        )
    }

    fn __str__(&self) -> String {
        format!("{} ({})", self.value, self.raw_kind)
    }
}

impl From<odict::Pronunciation> for Pronunciation {
    fn from(pronunciation: odict::Pronunciation) -> Self {
        let odict::Pronunciation { kind, value, urls } = pronunciation;

        Self {
            kind: PronunciationKind::from(kind),
            value,
            urls: urls.into_iter().map(MediaURL::from).collect(),
        }
    }
}
