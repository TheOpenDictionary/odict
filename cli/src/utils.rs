use std::time::Duration;

use odict::{lookup::LookupResult, ArchivedEntry, Entry};

pub fn t<F>(cb: F) -> anyhow::Result<Duration>
where
    F: FnOnce() -> anyhow::Result<()>,
{
    let start = std::time::Instant::now();

    cb()?;

    let end = std::time::Instant::now();

    Ok(end.duration_since(start))
}

pub fn get_lookup_entries(results: Vec<LookupResult<&ArchivedEntry>>) -> Vec<Entry> {
    results
        .iter()
        .map(|result| result.entry.to_entry().unwrap())
        .collect()
}
