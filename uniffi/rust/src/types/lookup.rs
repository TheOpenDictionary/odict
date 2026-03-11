use super::entry::Entry;

#[derive(uniffi::Record, Debug, Clone)]
pub struct LookupResult {
    pub entry: Entry,
    pub directed_from: Option<Entry>,
}

impl LookupResult {
    pub fn from_archive(result: &odict::lookup::LookupResult<&odict::schema::ArchivedEntry>) -> Result<Self, odict::Error> {
        let entry: Entry = result.entry.deserialize()?.into();
        let directed_from = match &result.directed_from {
            Some(e) => Some((*e).deserialize()?.into()),
            None => None,
        };
        Ok(Self { entry, directed_from })
    }
}

#[derive(uniffi::Record, Default)]
pub struct LookupOptions {
    pub split: Option<u32>,
    pub follow: Option<bool>,
    pub insensitive: Option<bool>,
}

impl From<LookupOptions> for odict::lookup::LookupOptions {
    fn from(opts: LookupOptions) -> Self {
        let mut o = Self::default();
        if let Some(s) = opts.split {
            if s > 0 {
                o = o.strategy(odict::lookup::LookupStrategy::Split(s as usize));
            }
        }
        if let Some(f) = opts.follow { o = o.follow(f); }
        if let Some(i) = opts.insensitive { o = o.insensitive(i); }
        o
    }
}
