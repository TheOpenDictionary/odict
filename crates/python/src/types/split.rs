use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct SplitOptions {
    #[pyo3(get, set)]
    pub min_length: Option<u32>,

    #[pyo3(get, set)]
    pub follow: Option<bool>,

    #[pyo3(get, set)]
    pub insensitive: Option<bool>,
}

#[pymethods]
impl SplitOptions {
    #[new]
    #[pyo3(signature = (min_length=None, follow=None, insensitive=None))]
    pub fn new(min_length: Option<u32>, follow: Option<bool>, insensitive: Option<bool>) -> Self {
        SplitOptions {
            min_length,
            follow,
            insensitive,
        }
    }
}

impl Default for SplitOptions {
    fn default() -> Self {
        SplitOptions {
            min_length: None,
            follow: None,
            insensitive: None,
        }
    }
}

impl From<SplitOptions> for odict::split::SplitOptions {
    fn from(opts: SplitOptions) -> Self {
        let mut options = odict::split::SplitOptions::default();

        if let Some(min_length) = opts.min_length {
            options = options.threshold(min_length as usize);
        }

        if let Some(follow) = opts.follow {
            options = options.follow(follow);
        }

        if let Some(insensitive) = opts.insensitive {
            options = options.insensitive(insensitive);
        }

        options
    }
}
