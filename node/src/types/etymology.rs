#[napi(object)]
pub struct JSEtymology {
  pub id: Option<String>,
  pub pronunciation: Option<String>,
  pub description: Option<String>,
  // pub senses: HashMap<PartOfSpeech, SenseJSON>,
}

impl JSEtymology {
  pub fn from(etymology: odict::Etymology, mds: &odict::MarkdownStrategy) -> Self {
    let odict::Etymology {
      id,
      pronunciation,
      description,..
      // senses,
    } = etymology;

    Self {
      id,
      pronunciation,
      description: description.map(|d| d.parse(mds)),
      // senses: senses
      //   .into_iter()
      //   .map(|(k, v)| (k, SenseJSON::from(v)))
      //   .collect(),
    }
  }
}
