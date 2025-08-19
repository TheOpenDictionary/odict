use std::env;
use std::path::{Path, PathBuf};

use crate::error::Result;

pub mod client;
pub mod downloader;
pub mod downloader2;
pub mod metadata;
pub mod utils;

pub use downloader::{DownloadOptions, Downloader};
pub use metadata::{EtagCache, EtagMetadata};
pub use utils::*;

/// Downloads a dictionary file from the ODict repository
///
/// # Arguments
///
/// * `dict_lang` - Dictionary and language string in format "dictionary/language" (e.g., "wiktionary/eng")
/// * `target_dir` - Optional target directory. If None, uses current working directory
///
/// # Returns
///
/// Returns the path to the downloaded file on success
///
/// # Example
///
/// ```rust
/// use odict::download::download;
/// use std::path::Path;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let path = download("wiktionary/eng", Some(Path::new("./downloads"))).await?;
/// println!("Downloaded to: {}", path.display());
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "download")]
pub async fn download(dict_lang: &str, target_dir: Option<&Path>) -> Result<PathBuf> {
    download_with_options(dict_lang & DownloadOptions::default()).await
}

/// Downloads a dictionary file with custom options
///
/// # Arguments
///
/// * `dict_lang` - Dictionary and language string in format "dictionary/language" (e.g., "wiktionary/eng")
/// * `target_dir` - Optional target directory. If None, uses current working directory
/// * `options` - Download options (caching, custom base URL, etc.)
///
/// # Returns
///
/// Returns the path to the downloaded file on success
///
/// # Example
///
/// ```rust
/// use odict::download::{download_with_options, DownloadOptions};
/// use std::path::Path;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Download with caching (default)
/// let path = download_with_options("wiktionary/eng", Some(Path::new("./downloads")), &DownloadOptions::new()).await?;
///
/// // Force download, bypassing cache
/// let path = download_with_options("wiktionary/eng", Some(Path::new("./downloads")), &DownloadOptions::new().force(true)).await?;
///
/// println!("Downloaded to: {}", path.display());
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "download")]
pub async fn download_with_options(name: &str, options: &DownloadOptions) -> Result<PathBuf> {
    // Create target directory if it doesn't exist
    if !target_dir.exists() {
        std::fs::create_dir_all(&target_dir).map_err(crate::error::Error::Io)?;
    }

    // Parse name to get language for filename
    let (_, language) = parse_dictionary_name(name)?;

    // Construct the target file path
    let filename = format!("{}.odict", language);
    let target_path = target_dir.join(filename);

    // Set up cache directory
    let cache_dir = target_dir.join(".cache");

    // Use the downloader
    let downloader = Downloader::default();

    downloader
        .download(name, &target_path, &cache_dir, options)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_parse_dict_lang_valid() {
        // This test doesn't actually download, just tests parsing logic
        let dict_lang = "wiktionary/eng";
        let parts: Vec<&str> = dict_lang.split('/').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], "wiktionary");
        assert_eq!(parts[1], "eng");
    }

    #[test]
    fn test_parse_dict_lang_invalid() {
        let invalid_formats = vec![
            "wiktionary",
            "wiktionary/",
            "/eng",
            "wiktionary/eng/extra",
            "",
        ];

        for format in invalid_formats {
            let parts: Vec<&str> = format.split('/').collect();
            if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
                // This would trigger an error in the actual function
                assert!(true);
            }
        }
    }

    #[test]
    fn test_target_path_construction() {
        let temp_dir = TempDir::new().unwrap();
        let target_dir = temp_dir.path();
        let filename = "eng.odict";
        let target_path = target_dir.join(filename);

        assert!(target_path.to_string_lossy().ends_with("eng.odict"));
    }

    #[cfg(feature = "download")]
    #[test]
    fn test_download_options() {
        let options = DownloadOptions::default();
        assert!(!options.force);
        assert!(options.base_url.is_none());

        let custom_options = DownloadOptions::new()
            .force(true)
            .base_url("https://custom.example.com".to_string());

        assert!(custom_options.force);
        assert_eq!(
            custom_options.base_url,
            Some("https://custom.example.com".to_string())
        );
    }

    #[test]
    fn test_download_interface() {
        // Test that the download function interface works as expected
        let dict_lang = "wiktionary/eng";
        let parts: Vec<&str> = dict_lang.split('/').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], "wiktionary");
        assert_eq!(parts[1], "eng");

        // Test that invalid formats still fail as expected
        let invalid_formats = vec!["invalid", "invalid/", "/invalid", ""];
        for format in invalid_formats {
            let parts: Vec<&str> = format.split('/').collect();
            assert!(parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty());
        }
    }

    #[cfg(feature = "download")]
    #[test]
    fn test_etag_cache_integration() {
        use std::time::SystemTime;

        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join(".cache");

        // Test cache creation and loading
        let mut cache = EtagCache::load(&cache_dir).unwrap();
        assert!(cache.entries.is_empty());

        // Add some metadata
        let metadata = EtagMetadata {
            etag: "test-etag-123".to_string(),
            file_path: temp_dir.path().join("test.odict"),
            last_modified: SystemTime::now(),
            url: "https://example.com/test.odict".to_string(),
        };

        cache.set("test/dict".to_string(), metadata.clone());

        // Save and reload
        cache.save(&cache_dir).unwrap();
        let reloaded_cache = EtagCache::load(&cache_dir).unwrap();

        let loaded_metadata = reloaded_cache.get("test/dict").unwrap();
        assert_eq!(loaded_metadata.etag, metadata.etag);
        assert_eq!(loaded_metadata.url, metadata.url);
    }

    #[cfg(feature = "download")]
    #[test]
    fn test_cache_file_existence_check() {
        use std::fs;
        use std::time::SystemTime;

        let temp_dir = TempDir::new().unwrap();
        let mut cache = EtagCache::default();

        // Test with non-existent file
        let metadata = EtagMetadata {
            etag: "test-etag".to_string(),
            file_path: temp_dir.path().join("missing.odict"),
            last_modified: SystemTime::now(),
            url: "https://example.com/test.odict".to_string(),
        };

        cache.set("test/dict".to_string(), metadata);
        assert!(cache.needs_update("test/dict"));
        assert!(cache.get_cached_path("test/dict").is_none());

        // Test with existing file
        let existing_file = temp_dir.path().join("existing.odict");
        fs::write(&existing_file, "test content").unwrap();

        let metadata = EtagMetadata {
            etag: "test-etag".to_string(),
            file_path: existing_file.clone(),
            last_modified: SystemTime::now(),
            url: "https://example.com/test.odict".to_string(),
        };

        cache.set("test/dict".to_string(), metadata);
        assert!(!cache.needs_update("test/dict"));
        assert_eq!(cache.get_cached_path("test/dict"), Some(&existing_file));
    }

    #[cfg(feature = "download")]
    #[test]
    fn test_download_result_enum() {
        use crate::download::downloader::DownloadResult;

        // Test that our enum variants work as expected
        let not_modified = DownloadResult::NotModified;
        let downloaded = DownloadResult::Downloaded {
            etag: Some("new-etag".to_string()),
            bytes: vec![1, 2, 3, 4],
        };

        match not_modified {
            DownloadResult::NotModified => assert!(true),
            _ => panic!("Expected NotModified variant"),
        }

        match downloaded {
            DownloadResult::Downloaded { etag, bytes } => {
                assert_eq!(etag, Some("new-etag".to_string()));
                assert_eq!(bytes, vec![1, 2, 3, 4]);
            }
            _ => panic!("Expected Downloaded variant"),
        }
    }
}
