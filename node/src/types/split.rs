#[napi(object)]
#[derive(Clone, Default)]
pub struct SplitOptions {
    pub min_length: Option<u32>,
    pub follow: Option<bool>,
    pub insensitive: Option<bool>,
}

impl From<SplitOptions> for odict::split::SplitOptions {
    fn from(opts: SplitOptions) -> Self {
        let mut options = odict::split::SplitOptions::default();

        if let Some(min_length) = opts.min_length {
            options = options.threshold(min_length as usize);
        }

        if let Some(follow) = opts.follow {
            options = options.follow(follow);
        }

        if let Some(insensitive) = opts.insensitive {
            options = options.insensitive(insensitive);
        }

        options
    }
}
