use super::Etymology;

#[napi(object)]
pub struct Entry {
  pub term: String,
  pub see_also: Option<String>,
  pub etymologies: Vec<Etymology>,
}

impl Entry {
  pub fn from_entry(entry: odict::Entry, mds: &odict::MarkdownStrategy) -> Self {
    let odict::Entry {
      term,
      see_also,
      etymologies,
    } = entry;

    Self {
      term,
      see_also,
      etymologies: etymologies
        .into_iter()
        .map(|e| Etymology::from(e, mds))
        .collect(),
    }
  }

  pub fn from_archive(entry: &odict::ArchivedEntry, mds: &odict::MarkdownStrategy) -> Self {
    Entry::from_entry(entry.to_entry().unwrap(), mds)
  }
}
