use crate::{
    format::md::ToMarkdown,
    md::to_html,
    schema::{ArchivedEntry, Entry},
};

pub trait ToHTML {
    fn to_html(self) -> crate::Result<String>;
}

impl<T: AsRef<Entry>> ToHTML for T {
    fn to_html(self) -> crate::Result<String> {
        Ok(to_html(self.to_markdown()?.as_str()))
    }
}

impl<T: AsRef<Entry>> ToHTML for Vec<T> {
    fn to_html(self) -> crate::Result<String> {
        let mut buffer = String::new();

        for entry in self {
            buffer.push_str(&entry.to_html()?);
            buffer.push_str("\n\n");
        }

        Ok(buffer)
    }
}

impl ToHTML for &ArchivedEntry {
    fn to_html(self) -> crate::Result<String> {
        Ok(to_html(self.to_markdown()?.as_str()))
    }
}

impl ToHTML for Vec<&ArchivedEntry> {
    fn to_html(self) -> crate::Result<String> {
        self.iter()
            .map(|entry| entry.to_entry().unwrap())
            .collect::<Vec<Entry>>()
            .to_html()
    }
}
