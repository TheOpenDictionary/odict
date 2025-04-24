use pyo3::prelude::*;
use structural_convert::StructuralConvert;

use super::media_url::MediaURL;
use super::pronunciation_kind::PronunciationKind;

#[pyclass]
#[derive(Clone, Debug, StructuralConvert)]
#[convert(from(odict::Pronunciation))]
pub struct Pronunciation {
    #[pyo3(get)]
    pub kind: PronunciationKind,

    #[pyo3(get)]
    pub value: String,

    #[pyo3(get)]
    pub media: Vec<MediaURL>,
}

#[pymethods]
impl Pronunciation {
    #[new]
    pub fn new(kind: &str, value: String) -> Self {
        Self {
            kind: PronunciationKind::new(kind),
            value,
            media: Vec::new(),
        }
    }

    pub fn add_media(&mut self, url: MediaURL) {
        self.media.push(url);
    }

    fn __repr__(&self) -> String {
        format!(
            "Pronunciation(kind='{}', value='{}', urls={})",
            self.kind,
            self.value,
            if self.media.is_empty() {
                "[]".to_string()
            } else {
                format!(
                    "[{}]",
                    self.media
                        .iter()
                        .map(|url| url.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        )
    }

    fn __str__(&self) -> String {
        format!("{} ({})", self.value, self.kind)
    }
}
