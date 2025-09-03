use crate::download::{parse_remote_dictionary_name, DictionaryDownloader};

use crate::OpenDictionary;

#[derive(Default)]
pub struct RemoteOptions {
    downloader: DictionaryDownloader,
}

impl AsRef<RemoteOptions> for RemoteOptions {
    fn as_ref(&self) -> &RemoteOptions {
        self
    }
}

impl OpenDictionary {
    /// Attempts to download and load a dictionary if the input looks like a dictionary name.
    ///
    /// Dictionary names must be in the format "dictionary/language" (e.g., "wiktionary/en").
    pub async fn from_remote_with_options<Options: AsRef<RemoteOptions>>(
        dictionary: &str,
        options: Options,
    ) -> crate::Result<OpenDictionary> {
        if parse_remote_dictionary_name(dictionary).is_ok() {
            let path = options
                .as_ref()
                .downloader
                .download(dictionary)
                .await
                .map_err(|e| {
                    crate::Error::DownloadFailed(dictionary.to_string(), format!("{e}"))
                })?;

            return OpenDictionary::from_path(path);
        }
        Err(crate::Error::InvalidDictionaryName(dictionary.to_string()))
    }

    pub async fn from_remote(dictionary: &str) -> crate::Result<OpenDictionary> {
        Self::from_remote_with_options(dictionary, RemoteOptions::default()).await
    }
}
