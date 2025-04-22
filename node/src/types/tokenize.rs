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
  fn from(opts: TokenizeOptions) -> Self {
    let mut options = odict::lookup::TokenizeOptions::default();

    if let Some(follow) = opts.follow {
      options = options.follow(follow);
    }

    if let Some(insensitive) = opts.insensitive {
      options = options.insensitive(insensitive);
    }

    if let Some(allow_list) = opts.allow_list {
      options = options.allow_list(
        allow_list
          .into_iter()
          .map(|s| Language::from_code(s))
          .filter_map(|l| l)
          .collect::<Vec<Language>>(),
      );
    }

    options
  }
}
