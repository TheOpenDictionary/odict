use merge::Merge;
use napi::bindgen_prelude::*;
use odict::lookup::Language;

#[napi(object)]
#[derive(Merge, Clone)]
pub struct TokenizeOptions {
  #[napi(ts_type = "boolean | number")]
  #[merge(strategy = merge::option::overwrite_none)]
  pub follow: Option<Either<bool, u32>>,
  #[merge(strategy = merge::option::overwrite_none)]
  pub allow_list: Option<Vec<String>>,
  #[merge(strategy = merge::option::overwrite_none)]
  pub insensitive: Option<bool>,
}

impl Default for TokenizeOptions {
  fn default() -> Self {
    TokenizeOptions {
      follow: None,
      allow_list: None,
      insensitive: None,
    }
  }
}

impl From<TokenizeOptions> for odict::lookup::TokenizeOptions {
  fn from(opts: TokenizeOptions) -> Self {
    let mut options = odict::lookup::TokenizeOptions::default();

    if let Some(follow) = opts.follow {
      options = options.follow(match follow {
        Either::A(true) => u32::MAX,
        Either::A(false) => 0,
        Either::B(num) => num,
      });
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
