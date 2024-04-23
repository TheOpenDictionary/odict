use super::definition::Definition;

#[napi(object)]
pub struct Group {
  pub id: Option<String>,
  pub description: String,
  pub definitions: Vec<Definition>,
}

impl Group {
  fn from(group: odict::Group, mds: odict::MarkdownStrategy) -> Self {
    let odict::Group {
      id,
      description,
      definitions,
    } = group;

    Self {
      id,
      description: description.parse(mds),
      definitions: definitions
        .into_iter()
        .map(|d| DefinitionJSON::from(d))
        .collect(),
    }
  }
}
