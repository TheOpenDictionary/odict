use super::LookupResult;

#[napi(object)]
pub struct Token {
  pub lemma: String,
  pub language: Option<String>,
  pub entries: Vec<LookupResult>,
  pub kind: String,
  pub script: String,
  pub start: u16,
  pub end: u16,
}

impl From<odict::Token<odict::Entry>> for Token {
  fn from(token: odict::Token<odict::Entry>) -> Self {
    Token {
      lemma: token.lemma.clone(),
      language: token.language.map(|s| s.code().to_string()),
      script: token.script.name().to_string(),
      kind: format!("{:?}", token.kind),
      start: token.start as u16,
      end: token.end as u16,
      entries: token.entries.into_iter().map(LookupResult::from).collect(),
    }
  }
}
