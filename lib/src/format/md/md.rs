use crate::schema::{ArchivedEntry, Entry};

use super::{entry::write_entry, utils::divider};

pub trait ToMarkdown {
    fn to_markdown(self) -> crate::Result<String>;
}

impl<T: AsRef<Entry>> ToMarkdown for T {
    fn to_markdown(self) -> crate::Result<String> {
        write_entry(self.as_ref())
    }
}

impl<T: AsRef<Entry>> ToMarkdown for Vec<T> {
    fn to_markdown(self) -> crate::Result<String> {
        let mut buffer = String::new();

        for entry in self {
            buffer.push_str(&entry.to_markdown()?);
            buffer.push_str(&format!("\n{}\n", divider()));
        }

        Ok(buffer)
    }
}

impl ToMarkdown for &ArchivedEntry {
    fn to_markdown(self) -> crate::Result<String> {
        write_entry(&self.to_entry()?)
    }
}

impl ToMarkdown for Vec<&ArchivedEntry> {
    fn to_markdown(self) -> crate::Result<String> {
        self.iter()
            .map(|entry| entry.to_entry().unwrap())
            .collect::<Vec<Entry>>()
            .to_markdown()
    }
}
