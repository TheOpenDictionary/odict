#[napi(object)]
pub struct RemoteLoadOptions {
    pub out_dir: Option<String>,
    pub caching: Option<bool>,
    pub retries: Option<u32>,
}

impl Default for RemoteLoadOptions {
    fn default() -> Self {
        Self {
            out_dir: None,
            caching: None,
            retries: None,
        }
    }
}

#[napi(object)]
pub struct LoadOptions {
    pub config_dir: Option<String>,
    pub remote: Option<RemoteLoadOptions>,
}

impl Default for LoadOptions {
    fn default() -> Self {
        Self {
            config_dir: None,
            remote: None,
        }
    }
}

impl TryFrom<LoadOptions> for odict::LoadOptions<'_> {
    type Error = odict::Error;

    fn try_from(opts: LoadOptions) -> Result<Self, Self::Error> {
        let mut options = odict::LoadOptions::default();

        if let Some(config_dir) = opts.config_dir {
            options = options.with_config_dir(&config_dir)
        }

        if let Some(remote_opts) = opts.remote {
            let mut downloader = odict::download::DictionaryDownloader::default();

            if let Some(caching) = remote_opts.caching {
                downloader = downloader.with_caching(caching);
            }

            if let Some(out_dir) = remote_opts.out_dir {
                downloader = downloader.with_out_dir(out_dir);
            }

            if let Some(retries) = remote_opts.retries {
                downloader = downloader.with_retries(retries);
            }

            options = options.with_downloader(downloader);
        }

        Ok(options)
    }
}
