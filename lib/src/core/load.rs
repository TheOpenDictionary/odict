#[cfg(feature = "download")]
use std::path::Path;

use crate::{DictionaryFile, DictionaryReader};

#[cfg(feature = "alias")]
use crate::config::AliasManager;

#[cfg(feature = "download")]
use crate::download::{parse_dictionary_name, DictionaryDownloader};

/// A utility for loading dictionaries from various sources.
///
/// This struct provides a unified interface for loading dictionaries from:
/// - Local file paths
/// - Dictionary names (with download feature)
/// - Dictionary aliases (with alias feature)
///
/// # Examples
/// ```
/// use odict::DictionaryLoader;
///
/// // Basic usage
/// let loader = DictionaryLoader::default();
///
/// // Chainable configuration
/// let loader = DictionaryLoader::default()
///     .with_alias_manager(alias_manager)
///     .with_downloader(custom_downloader);
/// ```
#[derive(Debug, Clone)]
pub struct DictionaryLoader {
    reader: DictionaryReader,

    #[cfg(feature = "alias")]
    alias_manager: AliasManager,

    #[cfg(feature = "download")]
    downloader: DictionaryDownloader,
}

impl Default for DictionaryLoader {
    fn default() -> Self {
        Self {
            reader: DictionaryReader::default(),

            #[cfg(feature = "alias")]
            alias_manager: AliasManager::default(),

            #[cfg(feature = "download")]
            downloader: DictionaryDownloader::default(),
        }
    }
}

impl DictionaryLoader {
    /// Creates a new DictionaryLoader with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "alias")]
    /// Sets the alias manager for this loader (chainable).
    pub fn with_alias_manager(mut self, alias_manager: AliasManager) -> Self {
        self.alias_manager = alias_manager;
        self
    }

    #[cfg(feature = "download")]
    /// Sets the downloader for this loader (chainable).
    pub fn with_downloader(mut self, downloader: DictionaryDownloader) -> Self {
        self.downloader = downloader;
        self
    }

    /// Attempts to load from an alias if the alias feature is enabled.
    /// Returns None if no alias is found or alias feature is disabled.
    fn try_load_from_alias(&self, input: &str) -> Option<crate::Result<DictionaryFile>> {
        #[cfg(feature = "alias")]
        {
            if let Some(path) = self.alias_manager.get(input) {
                return Some(self.reader.read_from_path(path));
            }
        }
        #[cfg(not(feature = "alias"))]
        {
            let _ = input; // Suppress unused variable warning
        }
        None
    }

    /// Loads from a local file path.
    fn load_from_path(&self, path: &str) -> crate::Result<DictionaryFile> {
        self.reader.read_from_path(path)
    }

    #[cfg(feature = "download")]
    /// Attempts to download and load a dictionary if the input looks like a dictionary name.
    ///
    /// Dictionary names must be in the format "dictionary/language" (e.g., "wiktionary/en").
    async fn try_download(&self, input: &str) -> Option<crate::Result<DictionaryFile>> {
        // Try to parse as dictionary/language format
        if let Ok(_) = parse_dictionary_name(input) {
            let result = async {
                let bytes = self.downloader.download(input).await?;
                self.reader.read_from_bytes(bytes)
            }
            .await;
            return Some(result);
        }
        None
    }
}

// Conditional compilation for sync vs async load method
#[cfg(not(feature = "download"))]
impl DictionaryLoader {
    /// Loads a dictionary from a path or alias.
    ///
    /// # Arguments
    /// * `input` - Either a local file path or dictionary alias
    ///
    /// # Examples
    /// ```
    /// use odict::DictionaryLoader;
    ///
    /// let loader = DictionaryLoader::default();
    /// let dict = loader.load("path/to/dictionary.odict")?;
    ///
    /// // With custom configuration
    /// # #[cfg(feature = "alias")]
    /// let loader = DictionaryLoader::default()
    ///     .with_alias_manager(alias_manager);
    /// # Ok::<(), odict::Error>(())
    /// ```
    pub fn load(&self, input: &str) -> crate::Result<DictionaryFile> {
        // Try to load from alias first
        if let Some(result) = self.try_load_from_alias(input) {
            return result;
        }

        // Fall back to loading from file path
        self.load_from_path(input)
    }
}

#[cfg(feature = "download")]
impl DictionaryLoader {
    /// Loads a dictionary from a path, alias, or downloads it if it's a dictionary name.
    ///
    /// # Arguments
    /// * `input` - Can be:
    ///   - A local file path
    ///   - A dictionary alias (if alias feature is enabled)
    ///   - A dictionary name in format "dictionary/language"
    ///
    /// # Examples
    /// ```
    /// use odict::DictionaryLoader;
    ///
    /// # async fn example() -> Result<(), odict::Error> {
    /// let loader = DictionaryLoader::default();
    ///
    /// // Load from local path
    /// let dict1 = loader.load("path/to/dictionary.odict").await?;
    ///
    /// // Download from remote
    /// let dict2 = loader.load("wiktionary/en").await?;
    ///
    /// // With custom configuration
    /// let loader = DictionaryLoader::default()
    ///     .with_downloader(custom_downloader)
    ///     .with_alias_manager(alias_manager);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn load(&self, input: &str) -> crate::Result<DictionaryFile> {
        // Try to load from alias first
        if let Some(result) = self.try_load_from_alias(input) {
            return result;
        }

        // Try to load from local file path if it exists
        if Path::new(input).exists() {
            return self.load_from_path(input);
        }

        // Try to download if it looks like a dictionary name
        if let Some(result) = self.try_download(input).await {
            return result;
        }

        // Fall back to trying as a file path (will return appropriate error)
        self.load_from_path(input)
    }
}
