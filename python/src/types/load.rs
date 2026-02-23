use pyo3::prelude::*;

#[pyclass]
#[derive(PartialEq, Default, Clone, Eq)]
pub struct RemoteLoadOptions {
    #[pyo3(get, set)]
    pub out_dir: Option<String>,
    #[pyo3(get, set)]
    pub caching: Option<bool>,
    #[pyo3(get, set)]
    pub retries: Option<u32>,
}

#[pymethods]
impl RemoteLoadOptions {
    #[new]
    #[pyo3(signature = (out_dir=None, caching=None, retries=None))]
    pub fn new(out_dir: Option<String>, caching: Option<bool>, retries: Option<u32>) -> Self {
        RemoteLoadOptions { out_dir, caching, retries }
    }
}

#[pyclass]
#[derive(PartialEq, Default, Clone, Eq)]
pub struct LoadOptions {
    #[pyo3(get, set)]
    pub config_dir: Option<String>,
    #[pyo3(get, set)]
    pub remote: Option<RemoteLoadOptions>,
}

#[pymethods]
impl LoadOptions {
    #[new]
    #[pyo3(signature = (config_dir=None, remote=None))]
    pub fn new(config_dir: Option<String>, remote: Option<RemoteLoadOptions>) -> Self {
        LoadOptions { config_dir, remote }
    }
}

impl TryFrom<LoadOptions> for odict::LoadOptions<'_> {
    type Error = odict::Error;

    fn try_from(opts: LoadOptions) -> Result<Self, Self::Error> {
        let mut options = odict::LoadOptions::default();

        if let Some(config_dir) = opts.config_dir {
            options = options
                .with_config_dir(&config_dir)
                .with_alias_manager(odict::alias::AliasManager::new(&config_dir));
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
