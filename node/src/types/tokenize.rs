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

impl From<TokenizeOptions> for odict::lookup::TokenizeOptions {
  fn from(options: TokenizeOptions) -> Self {
    let mut opts = Self::default();

    if let Some(follow) = options.follow {
      opts = opts.follow(follow);
    }

    if let Some(languages) = options.allow_list {
      let langs: Vec<Language> = languages
        .iter()
        .filter_map(|code| Language::from_code(code))
        .collect();

      if !langs.is_empty() {
        opts = opts.allow_list(langs);
      }
    }

    if let Some(insensitive) = options.insensitive {
      opts = opts.insensitive(insensitive);
    }

    opts
  }
}
