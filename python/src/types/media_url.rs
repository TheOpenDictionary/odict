use pyo3::prelude::*;
use std::fmt;

#[pyclass]
#[derive(Clone, Debug)]
pub struct MediaURL {
    #[pyo3(get)]
    pub src: String,

    #[pyo3(get)]
    pub mime_type: Option<String>,

    #[pyo3(get)]
    pub description: Option<String>,
}

#[pymethods]
impl MediaURL {
    #[new]
    pub fn new(src: String, mime_type: Option<String>, description: Option<String>) -> Self {
        Self {
            src,
            mime_type,
            description,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "MediaURL(src='{}', mime_type={:?}, description={:?})",
            self.src, self.mime_type, self.description
        )
    }

    fn __str__(&self) -> String {
        self.src.clone()
    }
}

impl fmt::Display for MediaURL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.src)
    }
}

impl From<odict::MediaURL> for MediaURL {
    fn from(media_url: odict::MediaURL) -> Self {
        let odict::MediaURL {
            src,
            mime_type,
            description,
        } = media_url;

        Self {
            src,
            mime_type,
            description,
        }
    }
}
