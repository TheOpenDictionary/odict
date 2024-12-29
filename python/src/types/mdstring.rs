use pyo3::prelude::*;

#[pyclass(eq, eq_int)]
#[derive(PartialEq)]
pub enum MarkdownStrategy {
    Disabled,
    HTML,
    Text,
}

impl From<&MarkdownStrategy> for odict::MarkdownStrategy {
    fn from(strategy: &MarkdownStrategy) -> Self {
        match strategy {
            MarkdownStrategy::Disabled => odict::MarkdownStrategy::Disabled,
            MarkdownStrategy::HTML => odict::MarkdownStrategy::HTML,
            MarkdownStrategy::Text => odict::MarkdownStrategy::Text,
        }
    }
}

impl From<&str> for MarkdownStrategy {
    fn from(strategy: &str) -> Self {
        match strategy {
            "html" => MarkdownStrategy::HTML,
            "text" => MarkdownStrategy::Text,
            _ => MarkdownStrategy::Disabled,
        }
    }
}

#[pyclass]
#[derive(Debug)]
pub struct String {
    mds: odict::String,
}

#[pymethods]
impl String {
    #[new]
    pub fn new(value: String) -> PyResult<Self> {
        Ok(Self {
            mds: odict::String::from(value.as_str()),
        })
    }

    #[getter]
    pub fn value(&self) -> String {
        self.mds.value().to_string()
    }

    pub fn parse(&self, strategy: &MarkdownStrategy) -> String {
        self.mds.parse::<odict::MarkdownStrategy>(strategy.into())
    }
}

impl From<odict::String> for String {
    fn from(mds: odict::String) -> Self {
        Self { mds }
    }
}
