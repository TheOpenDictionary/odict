#[derive(uniffi::Record, Default)]
pub struct SearchOptions {
    pub limit: Option<u32>,
}

impl From<SearchOptions> for odict::search::SearchOptions {
    fn from(opts: SearchOptions) -> Self {
        let mut o = Self::default();
        if let Some(l) = opts.limit { o = o.limit(l as usize); }
        o
    }
}
