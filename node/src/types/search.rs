#[napi(object)]
#[derive(PartialEq, Clone, Eq, Default)]
#[cfg(feature = "node")]
pub struct SearchOptions {
    pub directory: Option<String>,
    pub threshold: Option<u32>,
    pub autoindex: Option<bool>,
    pub limit: Option<u32>,
}

#[cfg(feature = "node")]
impl From<SearchOptions> for odict::search::SearchOptions {
    fn from(opts: SearchOptions) -> Self {
        let mut options = odict::search::SearchOptions::default();

        if let Some(dir) = opts.directory {
            options = options.dir(dir.as_str());
        }

        if let Some(threshold) = opts.threshold {
            options = options.threshold(threshold);
        }

        if let Some(autoindex) = opts.autoindex {
            options = options.autoindex(autoindex);
        }

        if let Some(limit) = opts.limit {
            options = options.limit(limit as usize);
        }

        options
    }
}
