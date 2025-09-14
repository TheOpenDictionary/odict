use odict::{OpenDictionary, alias::AliasOptions};

pub struct LoadDictionaryOptions {
    alias_manager: Option<odict::alias::AliasManager>,
}

impl Default for LoadDictionaryOptions {
    fn default() -> Self {
        LoadDictionaryOptions {
            alias_manager: None,
        }
    }
}

impl LoadDictionaryOptions {
    pub fn with_alias_manager(mut self, manager: odict::alias::AliasManager) -> Self {
        self.alias_manager = Some(manager);
        self
    }
}

pub async fn load_dictionary_with_options(
    name: &str,
    options: LoadDictionaryOptions,
) -> odict::Result<OpenDictionary> {
    // Attempt to load first from the remote if the request matches the regex
    match OpenDictionary::from_remote(name).await {
        Ok(dictionary) => Ok(dictionary),
        // If the name is invalid, try loading from an alias
        Err(odict::Error::InvalidDictionaryName(_)) => {
            let opts: AliasOptions = match options.alias_manager {
                Some(manager) => AliasOptions::default().with_manager(manager),
                None => AliasOptions::default(),
            };

            match OpenDictionary::from_alias_with_options(name, opts) {
                Ok(dictionary) => Ok(dictionary),
                // If no alias found, try loading from path
                Err(odict::Error::AliasNotFound(_)) => OpenDictionary::from_path(name),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn load_dictionary(name: &str) -> odict::Result<OpenDictionary> {
    load_dictionary_with_options(name, LoadDictionaryOptions::default()).await
}
