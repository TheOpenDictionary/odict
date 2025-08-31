use merge::Merge;
use pyo3::prelude::*;

#[pyclass]
#[derive(Merge, Clone)]
pub struct IndexOptions {
    #[pyo3(get, set)]
    #[merge(strategy = merge::option::overwrite_none)]
    pub directory: Option<String>,

    #[pyo3(get, set)]
    #[merge(strategy = merge::option::overwrite_none)]
    pub memory: Option<usize>,

    #[pyo3(get, set)]
    #[merge(strategy = merge::option::overwrite_none)]
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

impl Default for IndexOptions {
    fn default() -> Self {
        IndexOptions {
            directory: None,
            memory: None,
            overwrite: None,
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
