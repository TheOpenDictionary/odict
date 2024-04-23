#[napi(object)]
pub struct Definition {
  pub id: Option<String>,
  pub value: String,
  pub examples: Vec<String>,
  pub notes: Vec<NoteJSON>,
}

impl Definition {
  fn from(definition: odict::Definition, mds: odict::MarkdownStrategy) -> Self {
    let odict::Definition {
      id,
      value,
      examples,
      notes,
    } = definition;

    Self {
      id,
      value: value.parse(mds),
      examples: examples.into_iter().map(|e| e.value.parse(mds)).collect(),
      notes: notes.into_iter().map(|n| NoteJSON::from(n)).collect(),
    }
  }
}
