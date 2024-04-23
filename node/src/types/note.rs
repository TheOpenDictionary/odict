#[napi(object)]
pub struct Note {
  pub id: Option<String>,
  pub value: String,
  pub examples: Vec<String>,
}

impl Note {
  pub fn from(note: odict::Note, mds: &odict::MarkdownStrategy) -> Self {
    let odict::Note {
      id,
      value,
      examples,
    } = note;

    Self {
      id,
      value: value.parse(mds),
      examples: examples.into_iter().map(|e| e.value.parse(mds)).collect(),
    }
  }
}
