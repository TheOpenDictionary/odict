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
