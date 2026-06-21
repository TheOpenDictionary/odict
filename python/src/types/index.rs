use pyo3::prelude::*;

/// Options for configuring full-text index creation.
#[pyclass(from_py_object)]
#[derive(Clone, Default)]
pub struct IndexOptions {
    /// Custom directory for storing the index.
    #[pyo3(get, set)]
    pub directory: Option<String>,

    /// Memory arena size per thread in bytes (must be >15 MB).
    #[pyo3(get, set)]
    pub memory: Option<usize>,

    /// Whether to overwrite an existing index.
    #[pyo3(get, set)]
    pub overwrite: Option<bool>,
}

#[pymethods]
impl IndexOptions {
    #[new]
    #[pyo3(signature = (directory=None, memory=None, overwrite=None))]
    pub fn new(directory: Option<String>, memory: Option<usize>, overwrite: Option<bool>) -> Self {
        IndexOptions {
            directory,
            memory,
            overwrite,
        }
    }
}

impl From<IndexOptions> for odict::index::IndexOptions {
    fn from(opts: IndexOptions) -> Self {
        let mut options = odict::index::IndexOptions::default();

        if let Some(directory) = opts.directory {
            options = options.dir(&directory);
        }

        if let Some(memory) = opts.memory {
            options = options.memory(memory);
        }

        if let Some(overwrite) = opts.overwrite {
            options = options.overwrite(overwrite);
        }

        options
    }
}
