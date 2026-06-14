#[cfg(feature = "config")]
use std::ffi::OsStr;

use std::marker::PhantomData;
#[cfg(feature = "config")]
use std::path::PathBuf;

#[cfg(feature = "alias")]
use crate::alias::AliasManager;

use crate::OpenDictionary;

#[cfg(feature = "config")]
use crate::config::DEFAULT_CONFIG_DIR;

#[cfg(feature = "http")]
use crate::download::DictionaryDownloader;

#[derive(Clone)]
pub struct LoadOptions<'a> {
    #[cfg(feature = "alias")]
    alias_manager: crate::alias::AliasManager,
    #[cfg(feature = "http")]
    downloader: crate::download::DictionaryDownloader<'a>,
    #[cfg(feature = "config")]
    config_dir: PathBuf,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Default for LoadOptions<'a> {
    fn default() -> Self {
        LoadOptions {
            #[cfg(feature = "config")]
            config_dir: DEFAULT_CONFIG_DIR.to_path_buf(),
            #[cfg(feature = "alias")]
            alias_manager: crate::alias::AliasManager::default(),
            #[cfg(feature = "http")]
            downloader: crate::download::DictionaryDownloader::default(),
            phantom: PhantomData,
        }
    }
}

impl<'a> LoadOptions<'a> {
    #[cfg(feature = "alias")]
    pub fn with_alias_manager(mut self, manager: AliasManager) -> Self {
        self.alias_manager = manager;
        self
    }

    #[cfg(feature = "config")]
    pub fn with_config_dir<D: AsRef<OsStr> + ?Sized>(mut self, dir: &D) -> Self {
        self.config_dir = PathBuf::from(dir);

        #[cfg(feature = "alias")]
        if self.alias_manager.config_dir.is_none() {
            self.alias_manager.config_dir = Some(self.config_dir.clone());
        }

        #[cfg(feature = "http")]
        if self.downloader.options.config_dir.is_none() {
            self.downloader.options.config_dir = Some(self.config_dir.clone());
        }

        self
    }

    #[cfg(feature = "http")]
    pub fn with_downloader(mut self, downloader: DictionaryDownloader<'a>) -> Self {
        self.downloader = downloader;
        self
    }
}

impl OpenDictionary {
    pub async fn load_with_options<'a>(
        name: &str,
        options: LoadOptions<'a>,
    ) -> crate::Result<OpenDictionary> {
        #[cfg(feature = "http")]
        match OpenDictionary::from_remote_with_downloader(name, options.downloader).await {
            Ok(dictionary) => {
                return Ok(dictionary);
            }
            // If the name is invalid, skip
            Err(crate::Error::InvalidDictionaryName(_)) => {}
            Err(e) => {
                return Err(e);
            }
        }

        #[cfg(feature = "alias")]
        match OpenDictionary::from_alias_with_manager(name, options.alias_manager) {
            Ok(dictionary) => {
                return Ok(dictionary);
            }
            // If no alias found, skip
            Err(crate::Error::AliasNotFound(_)) => {}
            Err(e) => {
                return Err(e);
            }
        }

        return OpenDictionary::from_path(name);
    }

    pub async fn load(name: &str) -> crate::Result<OpenDictionary> {
        Self::load_with_options(name, LoadOptions::default()).await
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::LoadOptions;

    #[test]
    fn test_config_set() {
        let path = PathBuf::from("some/config/dir");
        let options = LoadOptions::default().with_config_dir(&path);

        assert_eq!(options.downloader.options.config_dir, Some(path.clone()));
        assert_eq!(options.alias_manager.config_dir, Some(path.clone()));
    }
}
