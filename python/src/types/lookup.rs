use pyo3::prelude::*;

use super::Entry;

#[pyclass]
#[derive(Debug)]
pub struct LookupResult {
    pub entry: Entry,
    pub directed_from: Option<Entry>,
}

impl LookupResult {
    pub fn from_archive(
        result: &odict::lookup::LookupResult<&odict::ArchivedEntry>,
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
