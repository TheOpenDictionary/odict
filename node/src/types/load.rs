use odict::alias::AliasManager;

#[napi(object)]
#[derive(PartialEq, Default, Clone, Eq)]
pub struct AliasLoadOptions {
  pub path: Option<String>,
}

impl AliasLoadOptions {
  pub fn with_path(mut self, path: String) -> Self {
    self.path = Some(path);
    self
  }
}

#[napi(object)]
#[derive(PartialEq, Default, Clone, Eq)]
pub struct LoadOptions {
  pub alias: Option<AliasLoadOptions>,
}

impl TryFrom<LoadOptions> for internal::LoadDictionaryOptions {
  type Error = odict::Error;

  fn try_from(opts: LoadOptions) -> Result<Self, Self::Error> {
    let mut options = internal::LoadDictionaryOptions::default();

    if let Some(path) = opts.alias.and_then(|a| a.path) {
      options = options.with_alias_manager(AliasManager::new(&path)?);
    }

    Ok(options)
  }
}
