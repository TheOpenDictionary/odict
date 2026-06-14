use odict::tokenize::Language;

#[napi(object)]
#[derive(Clone)]
pub struct TokenizeOptions {
    pub follow: Option<bool>,
    pub allow_list: Option<Vec<String>>,
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

impl From<TokenizeOptions> for odict::tokenize::TokenizeOptions {
    fn from(opts: TokenizeOptions) -> Self {
        let mut options = odict::tokenize::TokenizeOptions::default();

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
