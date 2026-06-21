use odict::tokenize::Language;

#[napi(object)]
#[derive(Clone, Default)]
pub struct TokenizeOptions {
    pub follow: Option<bool>,
    pub allow_list: Option<Vec<String>>,
    pub insensitive: Option<bool>,
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
                    .filter_map(Language::from_code)
                    .collect::<Vec<Language>>(),
            );
        }

        options
    }
}
