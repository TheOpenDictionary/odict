use merge::Merge;
use odict::lookup::Language;

#[napi(object)]
#[derive(PartialEq, Merge, Clone, Eq)]
pub struct TokenizeOptions {
  pub follow: Option<bool>,
  pub allow_list: Option<Vec<String>>,
  pub insensitive: Option<bool>,
}

impl Default for TokenizeOptions {
  fn default() -> Self {
    TokenizeOptions {
      follow: Some(true),
      allow_list: None,
      insensitive: None,
    }
  }
}

impl From<odict::lookup::TokenizeOptions> for TokenizeOptions {
  fn from(options: odict::lookup::TokenizeOptions) -> Self {
    TokenizeOptions {
      follow: Some(options.follow),
      allow_list: options
        .allow_list
        .map(|list| list.iter().map(|s| s.to_string()).collect()),
      insensitive: Some(options.insensitive),
    }
  }
}
