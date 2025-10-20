use std::path::Path;

use crate::download::{parse_remote_dictionary_name, DictionaryDownloader, DownloadOptions};

use crate::OpenDictionary;

pub struct RemoteOptions<'a> {
    downloader: DictionaryDownloader,
    download_options: DownloadOptions<'a>,
}

impl Default for RemoteOptions<'_> {
    fn default() -> Self {
        Self {
            downloader: DictionaryDownloader::default(),
            download_options: DownloadOptions::default(),
        }
    }
}

impl<'a> RemoteOptions<'a> {
    pub fn with_downloader(mut self, downloader: DictionaryDownloader) -> Self {
        self.downloader = downloader;
        self
    }

    pub fn with_caching(mut self, caching: bool) -> Self {
        self.download_options = self.download_options.with_caching(caching);
        self
    }

    pub fn with_out_dir<P: AsRef<Path>>(mut self, out_dir: P) -> Self {
        self.download_options = self.download_options.with_out_dir(out_dir);
        self
    }

    pub fn on_progress<F>(mut self, callback: F) -> Self
    where
        F: Fn(u64, Option<u64>, f64) + Send + Sync + 'a,
    {
        self.download_options = self.download_options.on_progress(callback);
        self
    }
}

impl<'a> AsRef<RemoteOptions<'a>> for RemoteOptions<'a> {
    fn as_ref(&self) -> &RemoteOptions<'a> {
        self
    }
}

impl OpenDictionary {
    /// Attempts to download and load a dictionary if the input looks like a dictionary name.
    ///
    /// Dictionary names must be in the format "dictionary/language" (e.g., "wiktionary/en").
    pub async fn from_remote_with_options<'a, Options: AsRef<RemoteOptions<'a>>>(
        dictionary: &str,
        options: Options,
    ) -> crate::Result<OpenDictionary> {
        if parse_remote_dictionary_name(dictionary).is_ok() {
            let path = options
                .as_ref()
                .downloader
                .download_with_options(dictionary, &options.as_ref().download_options)
                .await
                .map_err(|e| match e {
                    crate::Error::DownloadFailed(kind, msg) => {
                        crate::Error::DownloadFailed(kind, msg)
                    }
                    _ => crate::Error::DownloadFailed(
                        crate::download::NetworkError::Network,
                        format!("Failed to download {dictionary}: {e}"),
                    ),
                })?;

            return OpenDictionary::from_path(path);
        }
        Err(crate::Error::InvalidDictionaryName(dictionary.to_string()))
    }

    pub async fn from_remote(dictionary: &str) -> crate::Result<OpenDictionary> {
        Self::from_remote_with_options(dictionary, RemoteOptions::default()).await
    }
}
