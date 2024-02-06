/* -------------------------------------------------------------------------- */
/*                                Split Options                               */
/* -------------------------------------------------------------------------- */

use std::error::Error;

use crate::{schema::EntryBuffer, DictionaryFile};

pub struct SplitOptions {
    threshold: usize,
}

impl SplitOptions {
    pub fn default() -> Self {
        Self { threshold: 0 }
    }

    pub fn threshold(mut self, threshold: usize) -> Self {
        self.threshold = threshold;
        self
    }
}

impl DictionaryFile<'_> {
    pub fn split(
        &self,
        query: &str,
        options: &SplitOptions,
    ) -> Result<Vec<EntryBuffer>, Box<dyn Error>> {
        let buf = self.as_buffer();

        let mut entries: Vec<EntryBuffer> = Vec::new();
        let mut start = 0;
        let mut end = query.len();

        let SplitOptions { threshold } = options;

        while start < end {
            let substr = &query[start..end];
            let entry_option = buf.entries_by_key(substr);

            if let Some(entry) = entry_option {
                entries.push(entry.clone());
            }

            if entry_option != None || substr.len() <= *threshold {
                start = end;
                end = query.len();
            } else {
                end -= 1;
            }
        }

        Ok(entries)
    }
}
