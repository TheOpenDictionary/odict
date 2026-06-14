use crate::download::{parse_remote_dictionary_name, DictionaryDownloader};

use crate::OpenDictionary;

impl OpenDictionary {
    /// Attempts to download and load a dictionary if the input looks like a dictionary name.
    ///
    /// Dictionary names must be in the format "dictionary/language" (e.g., "wiktionary/en").
    pub async fn from_remote_with_downloader<'a, Downloader: AsRef<DictionaryDownloader<'a>>>(
        dictionary: &str,
        downloader: Downloader,
    ) -> crate::Result<OpenDictionary> {
        if parse_remote_dictionary_name(dictionary).is_ok() {
            return downloader.as_ref().download(dictionary).await;
        }
        Err(crate::Error::InvalidDictionaryName(dictionary.to_string()))
    }

    pub async fn from_remote(dictionary: &str) -> crate::Result<OpenDictionary> {
        Self::from_remote_with_downloader(dictionary, DictionaryDownloader::default()).await
    }
}
