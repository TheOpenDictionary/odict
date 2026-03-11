#[derive(uniffi::Record, Default)]
pub struct IndexOptions {
    pub overwrite: Option<bool>,
}

impl From<IndexOptions> for odict::index::IndexOptions {
    fn from(opts: IndexOptions) -> Self {
        let mut o = Self::default();
        if let Some(ow) = opts.overwrite { o = o.overwrite(ow); }
        o
    }
}
