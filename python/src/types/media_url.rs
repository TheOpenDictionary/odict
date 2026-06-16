use pyo3::prelude::*;
use std::fmt;
use structural_convert::StructuralConvert;

/// A reference to an external media resource (audio, image, etc.).
#[pyclass]
#[derive(Clone, Debug, StructuralConvert)]
#[convert(from(odict::schema::MediaURL))]
pub struct MediaURL {
    /// URL or path to the media file.
    #[pyo3(get)]
    pub src: String,

    /// MIME type (e.g. `audio/mpeg`), or `None`.
    #[pyo3(get)]
    pub mime_type: Option<String>,

    /// Human-readable description of the media.
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
