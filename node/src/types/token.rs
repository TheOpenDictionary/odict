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

// We handle token conversion manually in dictionary.rs now
// This implementation was causing generic parameter errors
