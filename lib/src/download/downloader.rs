use std::path::{Path, PathBuf};
use std::time::SystemTime;

use super::client::get_client;
use super::metadata::{EtagCache, EtagMetadata};
use crate::config::get_config_dir;
use crate::download::parse_dictionary_name;
use crate::error::{Error, Result};

/// Options for configuring download behavior
#[derive(Debug, Clone)]
pub struct DownloadOptions {
    pub caching: bool,
}

impl Default for DownloadOptions {
    fn default() -> Self {
        Self { caching: true }
    }
}

impl DownloadOptions {
    /// Create new options with caching enabled (force = false)
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether to enable the cache
    pub fn caching(mut self, caching: bool) -> Self {
        self.caching = caching;
        self
    }
}

/// Result of a download operation
#[derive(Debug)]
pub enum DownloadResult {
    /// File was not modified (304 response)
    NotModified,
    /// File was downloaded successfully
    Downloaded {
        etag: Option<String>,
        bytes: Vec<u8>,
    },
}

/// Core downloader struct that handles HTTP operations
pub struct Downloader {
    base_url: String,
    cache_dir: String,
}

impl Default for Downloader {
    fn default() -> Self {
        Self {
            base_url: "https://pub-520e4751d2374bc5bc14265c6e02e06e.r2.dev".to_string(),
            cache_dir: get_config_dir().join("./cache").to_string(),
        }
    }
}

impl Downloader {
    /// Create a new downloader with custom default base URL
    pub fn new(base_url: String) -> Self {
        Self { base_url: base_url }
    }

    /// Download a dictionary file with the given options
    pub async fn download(
        &self,
        dict_lang: &str,
        target_path: &Path,
        options: &DownloadOptions,
    ) -> Result<PathBuf> {
        // Handle caching logic
        if !options.caching {
            return self.download_impl(dict_lang, target_path, options).await;
        } else {
            // Use cache if available
            self.download_with_cache(dict_lang, &url, target_path, cache_dir)
                .await
        }
    }

    async fn download_impl(
        &self,
        dict_lang: &str,
        target_path: &Path,
        opts: &DownloadOptions,
    ) -> Result<PathBuf> {
        let (dictionary, language) = parse_dictionary_name(dict_lang)?;

        let base_url = &self.base_url;
        let url = format!("{}/{}/{}.odict", base_url, dictionary, language);

        let (etag, bytes) = self.fetch_response(&url, None).await?;

        std::fs::write(target_path, bytes).map_err(Error::Io)?;

        self.update_cache(dict_lang, target_path, etag);

        Ok(target_path.to_path_buf())
    }

    async fn update_cache(&self, dict_lang: &str, target_path: &Path, etag: &str) -> Result<()> {
        let mut etag_cache = EtagCache::load(&self.cache_dir).unwrap_or_default();

        let metadata = EtagMetadata {
            etag: etag.to_string(),
            file_path: target_path.to_path_buf(),
            last_modified: SystemTime::now(),
            url: url.to_string(),
        };

        etag_cache.set(dict_lang.to_string(), metadata);
        etag_cache.save(&self.cache_dir)?;

        Ok(())
    }

    /// Core HTTP fetch method that handles both conditional and regular downloads
    async fn fetch_response<P: AsRef<Path>>(
        &self,
        url: &str,
        etag: Option<&str>,
    ) -> Result<(Option<String>, Vec<u8>)> {
        let client = get_client();
        let mut request = client.get(url);

        // Add conditional header if provided
        if let Some(tag) = etag {
            request = request.header("If-None-Match", tag);
        }

        let response = request
            .send()
            .await
            .map_err(|e| Error::Other(format!("Failed to fetch from {}: {}", url, e)))?;

        match response.status().as_u16() {
            304 => {
                // Not Modified - return empty bytes (caller should handle this case)
                Ok((None, Vec::new()))
            }
            200 => {
                // Success - extract etag and bytes
                let etag = super::extract_etag(&response);

                let bytes = response
                    .bytes()
                    .await
                    .map_err(|e| Error::Other(format!("Failed to read response body: {}", e)))?
                    .to_vec();

                Ok((etag, bytes))
            }
            status => Err(Error::Other(format!(
                "Unexpected response status from {}: {}",
                url, status
            ))),
        }
    }

    /// Download with cache checking
    async fn download_with_cache(
        &self,
        dict_lang: &str,
        url: &str,
        target_path: &Path,
        cache_dir: &Path,
    ) -> Result<PathBuf> {
        let mut etag_cache = EtagCache::load(cache_dir).unwrap_or_default();

        // Check if we have a cached version and if it still exists
        if let Some(cached_path) = etag_cache.get_cached_path(dict_lang) {
            if cached_path == target_path {
                // File exists and matches expected path, try conditional download
                if let Some(metadata) = etag_cache.get(dict_lang) {
                    let (etag, bytes) = self.fetch_with_etag(url, Some(&metadata.etag)).await?;

                    if bytes.is_empty() {
                        // 304 Not Modified - file hasn't changed
                        return Ok(target_path.to_path_buf());
                    } else {
                        // File was updated, save new content and update cache
                        std::fs::write(target_path, bytes).map_err(Error::Io)?;

                        if let Some(etag) = etag {
                            let new_metadata = EtagMetadata {
                                etag,
                                file_path: target_path.to_path_buf(),
                                last_modified: SystemTime::now(),
                                url: url.to_string(),
                            };

                            etag_cache.set(dict_lang.to_string(), new_metadata);
                            etag_cache.save(cache_dir).ok();
                        }

                        return Ok(target_path.to_path_buf());
                    }
                }
            }
        }

        // No cache or cache invalid, do full download
        let (etag, bytes) = self.fetch_with_etag(url, None).await?;

        std::fs::write(target_path, bytes).map_err(Error::Io)?;

        // Update cache
        self.update_cache(dict_lang, url, target_path, cache_dir, etag)
            .ok();

        Ok(target_path.to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_options_default() {
        let options = DownloadOptions::default();
        assert!(!options.force);
        assert!(options.base_url.is_none());
    }

    #[test]
    fn test_download_options_builder() {
        let options = DownloadOptions::new()
            .force(true)
            .base_url("https://custom.example.com".to_string());

        assert!(options.force);
        assert_eq!(
            options.base_url,
            Some("https://custom.example.com".to_string())
        );
    }

    #[test]
    fn test_downloader_parse_dict_lang() {
        let downloader = Downloader::default();

        // Valid format
        let (dict, lang) = downloader.parse_dict_lang("wiktionary/eng").unwrap();
        assert_eq!(dict, "wiktionary");
        assert_eq!(lang, "eng");

        // Invalid formats
        assert!(downloader.parse_dict_lang("invalid").is_err());
        assert!(downloader.parse_dict_lang("invalid/").is_err());
        assert!(downloader.parse_dict_lang("/invalid").is_err());
        assert!(downloader.parse_dict_lang("").is_err());
    }

    #[test]
    fn test_download_result_enum() {
        let not_modified = DownloadResult::NotModified;
        let downloaded = DownloadResult::Downloaded {
            etag: Some("test-etag".to_string()),
            bytes: vec![1, 2, 3, 4],
        };

        match not_modified {
            DownloadResult::NotModified => assert!(true),
            _ => panic!("Expected NotModified variant"),
        }

        match downloaded {
            DownloadResult::Downloaded { etag, bytes } => {
                assert_eq!(etag, Some("test-etag".to_string()));
                assert_eq!(bytes, vec![1, 2, 3, 4]);
            }
            _ => panic!("Expected Downloaded variant"),
        }
    }
}
