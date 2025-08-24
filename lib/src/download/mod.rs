use crate::error::Result;

pub mod client;
pub mod download;
pub mod metadata;
pub mod utils;

#[cfg(feature = "http")]
use crate::download::download::DEFAULT_DOWNLOADER;
use crate::OpenDictionary;

#[cfg(feature = "http")]
pub use download::{DictionaryDownloader, DownloadOptions};

pub use utils::*;

#[cfg(feature = "http")]
impl OpenDictionary {
    /// Attempts to download and load a dictionary if the input looks like a dictionary name.
    ///
    /// Dictionary names must be in the format "dictionary/language" (e.g., "wiktionary/en").
    pub async fn from_remote(dictionary: &str) -> crate::Result<OpenDictionary> {
        if let Ok(_) = parse_dictionary_name(dictionary) {
            let bytes = DEFAULT_DOWNLOADER.download(dictionary).await.map_err(|e| {
                crate::Error::DownloadFailed(dictionary.to_string(), format!("{}", e))
            })?;
            return Ok(OpenDictionary::from_bytes(bytes)?);
        }
        Err(crate::Error::InvalidDictionaryName(dictionary.to_string()))
    }
}
