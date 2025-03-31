use crate::{ArchivedDictionary, ArchivedEntry, Dictionary, Entry};

pub(super) fn split<'a, E, Options: AsRef<SplitOptions>>(
    entry_map: &'a HashMap<&str, E>,
    query: &str,
    options: Options,
) -> crate::Result<Vec<&E>> {
    let mut entries: Vec<&E> = Vec::new();
    let mut start = 0;
    let chars: Vec<_> = query.chars().collect();
    let mut end = chars.len();

    let SplitOptions { min_length } = options.as_ref();

    while start < end {
        let substr: String = chars[start..end].iter().collect();
        let entry = self.entries.get(substr.as_str());

        if entry.is_some() {
            entries.push(entry.unwrap());
        }

        if entry.is_some() || substr.len() <= *min_length {
            start = end;
            end = chars.len();
        } else {
            end -= 1;
        }
    }

    Ok(entries)
}
