use odict::alias::AliasManager;

#[napi(object)]
#[derive(Default)]
pub struct AliasLoadOptions {
    pub path: Option<String>,
}

#[napi(object)]
pub struct RemoteLoadOptions {
    pub out_dir: Option<String>,
    pub caching: Option<bool>,
}

impl Default for RemoteLoadOptions {
    fn default() -> Self {
        Self {
            out_dir: None,
            caching: None,
        }
    }
}

#[napi(object)]
pub struct LoadOptions {
    pub alias: Option<AliasLoadOptions>,
    pub remote: Option<RemoteLoadOptions>,
}

impl Default for LoadOptions {
    fn default() -> Self {
        Self {
            alias: None,
            remote: None,
        }
    }
}

impl TryFrom<LoadOptions> for internal::LoadDictionaryOptions<'_> {
    type Error = odict::Error;

    fn try_from(opts: LoadOptions) -> Result<Self, Self::Error> {
        let mut options = internal::LoadDictionaryOptions::default();

        if let Some(path) = opts.alias.and_then(|a| a.path) {
            options = options.with_alias_manager(AliasManager::new(&path)?);
        }

        if let Some(remote_opts) = opts.remote {
            let mut ro = odict::remote::RemoteOptions::default();

            if let Some(caching) = remote_opts.caching {
                ro = ro.caching(caching);
            }

            if let Some(out_dir) = remote_opts.out_dir {
                ro = ro.out_dir(out_dir);
            }

            options = options.with_remote_options(ro);
        }

        Ok(options)
    }
}
