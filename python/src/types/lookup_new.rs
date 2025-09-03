use either::Either;
use merge::Merge;
use pyo3::prelude::*;

#[pyclass]
#[derive(Merge, Clone)]
pub struct LookupOptions {
    #[pyo3(get, set)]
    #[merge(strategy = merge::option::overwrite_none)]
    pub split: Option<u32>,

    #[pyo3(get, set)]
    #[merge(strategy = merge::option::overwrite_none)]
    pub follow: Option<Either<bool, u32>>,

    #[pyo3(get, set)]
    #[merge(strategy = merge::option::overwrite_none)]
    pub insensitive: Option<bool>,
}

#[pymethods]
impl LookupOptions {
    #[new]
    #[pyo3(signature = (split=None, follow=None, insensitive=None))]
    pub fn new(
        split: Option<u32>,
        follow: Option<Either<bool, u32>>,
        insensitive: Option<bool>,
    ) -> Self {
        LookupOptions {
            split,
            follow,
            insensitive,
        }
    }
}

impl Default for LookupOptions {
    fn default() -> Self {
        LookupOptions {
            split: None,
            follow: None,
            insensitive: None,
        }
    }
}

impl From<LookupOptions> for odict::lookup::LookupOptions {
    fn from(opts: LookupOptions) -> Self {
        let mut options = odict::lookup::LookupOptions::default();

        if let Some(split) = opts.split {
            options = options.strategy(odict::lookup::LookupStrategy::Split(split as usize));
        }

        if let Some(follow) = opts.follow {
            options = options.follow(match follow {
                Either::Left(true) => u32::MAX,
                Either::Left(false) => 0,
                Either::Right(num) => num,
            });
        }

        if let Some(insensitive) = opts.insensitive {
            options = options.insensitive(insensitive);
        }

        options
    }
}

#[pyclass]
pub struct LookupResult {
    #[pyo3(get)]
    pub entry: crate::types::Entry,
    #[pyo3(get)]
    pub directed_from: Option<crate::types::Entry>,
}

#[pymethods]
impl LookupResult {
    #[new]
    pub fn new(entry: crate::types::Entry, directed_from: Option<crate::types::Entry>) -> Self {
        LookupResult {
            entry,
            directed_from,
        }
    }
}

impl From<odict::lookup::LookupResult<odict::schema::Entry>> for LookupResult {
    fn from(result: odict::lookup::LookupResult<odict::schema::Entry>) -> Self {
        let entry = crate::types::Entry::from(result.entry);
        let directed_from = result.directed_from.map(|s| crate::types::Entry::from(s));

        Self {
            entry,
            directed_from,
        }
    }
}

impl LookupResult {
    pub fn from_archive(
        result: &odict::lookup::LookupResult<&odict::schema::ArchivedEntry>,
    ) -> PyResult<Self> {
        use crate::utils::cast_error;

        let entry = crate::types::Entry::from(result.entry.deserialize().map_err(cast_error)?);

        let directed_from = match result.directed_from {
            Some(e) => Some(crate::types::Entry::from(
                e.deserialize().map_err(cast_error)?,
            )),
            None => None,
        };

        Ok(LookupResult {
            entry,
            directed_from,
        })
    }
}
