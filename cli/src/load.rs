use odict::OpenDictionary;

pub async fn load_dictionary(name: &str) -> odict::Result<OpenDictionary> {
    // Attempt to load first from the remote if the request matches the regex
    match OpenDictionary::from_remote(name).await {
        Ok(dictionary) => Ok(dictionary),
        // If the name is invalid, try loading from an alias
        Err(odict::Error::InvalidDictionaryName(_)) => match OpenDictionary::from_alias(name) {
            Ok(dictionary) => Ok(dictionary),
            // If no alias found, try loading from path
            Err(odict::Error::AliasNotFound(_)) => OpenDictionary::from_path(name),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
