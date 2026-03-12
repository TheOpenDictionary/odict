use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct SearchOptions {
    #[pyo3(get, set)]
    pub directory: Option<String>,

    #[pyo3(get, set)]
    pub threshold: Option<u32>,

    #[pyo3(get, set)]
    pub autoindex: Option<bool>,

    #[pyo3(get, set)]
    pub limit: Option<usize>,
}

#[pymethods]
impl SearchOptions {
    #[new]
    #[pyo3(signature = (directory=None, threshold=None, autoindex=None, limit=None))]
    pub fn new(
        directory: Option<String>,
        threshold: Option<u32>,
        autoindex: Option<bool>,
        limit: Option<usize>,
    ) -> Self {
        SearchOptions {
            directory,
            threshold,
            autoindex,
            limit,
        }
    }
}

impl Default for SearchOptions {
    fn default() -> Self {
        SearchOptions {
            directory: None,
            threshold: None,
            autoindex: None,
            limit: None,
        }
    }
}

impl From<SearchOptions> for odict::search::SearchOptions {
    fn from(opts: SearchOptions) -> Self {
        let mut options = odict::search::SearchOptions::default();

        if let Some(directory) = opts.directory {
            options = options.dir(&directory);
        }

        if let Some(threshold) = opts.threshold {
            options = options.threshold(threshold);
        }

        if let Some(autoindex) = opts.autoindex {
            options = options.autoindex(autoindex);
        }

        if let Some(limit) = opts.limit {
            options = options.limit(limit);
        }

        options
    }
}
