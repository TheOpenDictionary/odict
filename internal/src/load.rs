use odict::{
    OpenDictionary,
    alias::{AliasManager, AliasOptions},
    remote::RemoteOptions,
};

pub struct LoadDictionaryOptions<'a> {
    alias_options: odict::alias::AliasOptions,
    remote_options: odict::remote::RemoteOptions<'a>,
}

impl<'a> Default for LoadDictionaryOptions<'a> {
    fn default() -> Self {
        LoadDictionaryOptions {
            alias_options: AliasOptions::default(),
            remote_options: RemoteOptions::default(),
        }
    }
}

impl<'a> LoadDictionaryOptions<'a> {
    pub fn with_alias_manager(mut self, manager: AliasManager) -> Self {
        self.alias_options = AliasOptions::default().with_manager(manager);
        self
    }

    pub fn with_remote_options(mut self, options: RemoteOptions<'a>) -> Self {
        self.remote_options = options;
        self
    }
}

pub async fn load_dictionary_with_options<'a>(
    name: &str,
    options: LoadDictionaryOptions<'a>,
) -> odict::Result<OpenDictionary> {
    // Attempt to load first from the remote if the request matches the regex
    match OpenDictionary::from_remote_with_options(name, options.remote_options).await {
        Ok(dictionary) => Ok(dictionary),
        // If the name is invalid, try loading from an alias
        Err(odict::Error::InvalidDictionaryName(_)) => {
            match OpenDictionary::from_alias_with_options(name, options.alias_options) {
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
