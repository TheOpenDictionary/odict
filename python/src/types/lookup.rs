use pyo3::prelude::*;

use super::Entry;

#[pyclass]
#[derive(Debug, Clone)]
pub struct LookupResult {
    #[pyo3(get)]
    pub entry: Entry,
    #[pyo3(get)]
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

impl From<odict::lookup::LookupResult<odict::Entry>> for LookupResult {
    fn from(result: odict::lookup::LookupResult<odict::Entry>) -> Self {
        let entry = Entry::from(result.entry);
        let directed_from = result.directed_from.map(Entry::from);

        Self {
            entry,
            directed_from,
        }
    }
}

impl From<odict::lookup::LookupResult<&odict::ArchivedEntry>> for LookupResult {
    fn from(result: odict::lookup::LookupResult<&odict::ArchivedEntry>) -> Self {
        let entry = Entry::from(result.entry.to_entry().unwrap());

        let directed_from = result
            .directed_from
            .map(|s| Entry::from(s.to_entry().unwrap()));

        Self {
            entry,
            directed_from,
        }
    }
}
