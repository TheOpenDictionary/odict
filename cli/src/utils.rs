use odict::{ArchivedEntry, Entry};

pub fn deserialize_nested_entries(entries: Vec<Vec<&ArchivedEntry>>) -> Vec<Vec<Entry>> {
    entries
        .iter()
        .map(|entries_inner| -> Vec<Entry> {
            entries_inner
                .iter()
                .map(|entry| entry.to_entry().unwrap())
                .collect()
        })
        .collect()
}
