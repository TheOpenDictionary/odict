use either::Either;
use merge::Merge;
use pyo3::prelude::*;

use super::Entry;

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
#[derive(Debug, Clone)]
pub struct LookupResult {
    #[pyo3(get)]
    pub entry: Entry,
    #[pyo3(get)]
    pub directed_from: Option<Entry>,
}

#[pymethods]
impl LookupResult {
    #[new]
    pub fn new(entry: Entry, directed_from: Option<Entry>) -> Self {
        LookupResult {
            entry,
            directed_from,
        }
    }
}

impl LookupResult {
    pub fn from_archive(
        result: &odict::lookup::LookupResult<&odict::schema::ArchivedEntry>,
    ) -> PyResult<Self> {
        let entry = Entry::from_archive(result.entry)?;
        let directed_from = match &result.directed_from {
            Some(from) => Some(Entry::from_archive(from)?),
            None => None,
        };

        Ok(Self {
            entry,
            directed_from,
        })
    }
}

impl From<odict::lookup::LookupResult<&odict::schema::ArchivedEntry>> for LookupResult {
    fn from(result: odict::lookup::LookupResult<&odict::schema::ArchivedEntry>) -> Self {
        let entry = Entry::from(result.entry.deserialize().unwrap());

        let directed_from = result
            .directed_from
            .map(|s| Entry::from(s.deserialize().unwrap()));

        Self {
            entry,
            directed_from,
        }
    }
}

impl From<odict::lookup::LookupResult<odict::schema::Entry>> for LookupResult {
    fn from(result: odict::lookup::LookupResult<odict::schema::Entry>) -> Self {
        let entry = Entry::from(result.entry);
        let directed_from = result.directed_from.map(|s| Entry::from(s));

        Self {
            entry,
            directed_from,
        }
    }
}
