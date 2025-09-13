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

    pub fn with_path(&mut self, path: String) -> Self {
        self.path = Some(path);
        self.clone()
    }
}

#[pyclass]
#[derive(PartialEq, Default, Clone, Eq)]
pub struct LoadOptions {
    #[pyo3(get, set)]
    pub alias: Option<AliasLoadOptions>,
}

#[pymethods]
impl LoadOptions {
    #[new]
    #[pyo3(signature = (alias=None))]
    pub fn new(alias: Option<AliasLoadOptions>) -> Self {
        LoadOptions { alias }
    }
}

impl TryFrom<LoadOptions> for internal::LoadDictionaryOptions {
    type Error = odict::Error;

    fn try_from(opts: LoadOptions) -> Result<Self, Self::Error> {
        let mut options = internal::LoadDictionaryOptions::default();

        if let Some(path) = opts.alias.and_then(|a| a.path) {
            options = options.with_alias_manager(odict::alias::AliasManager::new(&path)?);
        }

        Ok(options)
    }
}
