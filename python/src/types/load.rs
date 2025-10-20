use pyo3::prelude::*;

#[pyclass]
#[derive(PartialEq, Default, Clone, Eq)]
pub struct AliasLoadOptions {
    #[pyo3(get, set)]
    pub path: Option<String>,
}

#[pymethods]
impl AliasLoadOptions {
    #[new]
    #[pyo3(signature = (path=None))]
    pub fn new(path: Option<String>) -> Self {
        AliasLoadOptions { path }
    }
}

#[pyclass]
#[derive(PartialEq, Default, Clone, Eq)]
pub struct RemoteLoadOptions {
    #[pyo3(get, set)]
    pub out_dir: Option<String>,
    #[pyo3(get, set)]
    pub caching: Option<bool>,
}

#[pymethods]
impl RemoteLoadOptions {
    #[new]
    #[pyo3(signature = (out_dir=None, caching=None))]
    pub fn new(out_dir: Option<String>, caching: Option<bool>) -> Self {
        RemoteLoadOptions { out_dir, caching }
    }
}

#[pyclass]
#[derive(PartialEq, Default, Clone, Eq)]
pub struct LoadOptions {
    #[pyo3(get, set)]
    pub alias: Option<AliasLoadOptions>,
    #[pyo3(get, set)]
    pub remote: Option<RemoteLoadOptions>,
}

#[pymethods]
impl LoadOptions {
    #[new]
    #[pyo3(signature = (alias=None, remote=None))]
    pub fn new(alias: Option<AliasLoadOptions>, remote: Option<RemoteLoadOptions>) -> Self {
        LoadOptions { alias, remote }
    }
}

impl TryFrom<LoadOptions> for internal::LoadDictionaryOptions<'_> {
    type Error = odict::Error;

    fn try_from(opts: LoadOptions) -> Result<Self, Self::Error> {
        let mut options = internal::LoadDictionaryOptions::default();

        if let Some(path) = opts.alias.and_then(|a| a.path) {
            options = options.with_alias_manager(odict::alias::AliasManager::new(&path)?);
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
