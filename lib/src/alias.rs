use serde_json::to_vec;
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, read_to_string},
    path::PathBuf,
    sync::OnceLock,
};

use crate::{config::DEFAULT_CONFIG_DIR, OpenDictionary};

pub(crate) const DEFAULT_ALIAS_FILE: &str = "aliases.json";

#[derive(Debug, Clone)]
pub struct AliasManager {
    pub(crate) config_dir: Option<PathBuf>,
    aliases: OnceLock<HashMap<String, String>>,
}

impl AsRef<AliasManager> for AliasManager {
    fn as_ref(&self) -> &AliasManager {
        self
    }
}

impl AliasManager {
    pub fn new<S: AsRef<OsStr> + ?Sized>(config_dir: &S) -> Self {
        Self {
            config_dir: Some(PathBuf::from(config_dir)),
            aliases: OnceLock::new(),
        }
    }
}

impl Default for AliasManager {
    fn default() -> Self {
        Self {
            config_dir: None,
            aliases: OnceLock::new(),
        }
    }
}

impl AsMut<AliasManager> for AliasManager {
    fn as_mut(&mut self) -> &mut AliasManager {
        self
    }
}

impl AliasManager {
    fn save_to_disk(&mut self) -> crate::Result<()> {
        let config_bytes = to_vec(&self.get_aliases()?.clone())?;
        fs::write(&self.path(), config_bytes)?;
        Ok(())
    }

    fn path(&self) -> PathBuf {
        PathBuf::from(
            self.config_dir
                .clone()
                .unwrap_or(DEFAULT_CONFIG_DIR.to_path_buf())
                .clone(),
        )
        .join(DEFAULT_ALIAS_FILE)
    }

    fn get_aliases(&mut self) -> crate::Result<&mut HashMap<String, String>> {
        let path = self.path();

        if self.aliases.get().is_none() {
            let aliases = if !path.exists() {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }

                fs::write(&path, "{}").unwrap();

                HashMap::new()
            } else {
                let config = read_to_string(&path)?;
                serde_json::from_str(&config)?
            };

            self.aliases
                .set(aliases)
                .map_err(|e| crate::Error::Other(format!("{:?}", e)))?;
        }

        Ok(self.aliases.get_mut().unwrap())
    }

    pub fn add(&mut self, alias: &str, file: &OpenDictionary) -> crate::Result<()> {
        if self.get(alias)?.is_none() {
            self.set(alias, file)
        } else {
            Err(crate::Error::AliasExists)
        }
    }

    pub fn set(&mut self, alias: &str, file: &OpenDictionary) -> crate::Result<()> {
        match &file.path() {
            Some(path) => {
                self.get_aliases()?
                    .insert(alias.to_string(), path.to_string_lossy().to_string());
                self.save_to_disk()
            }
            None => Err(crate::Error::DictionaryMissingPath),
        }
    }

    pub fn delete(&mut self, alias: &str) -> crate::Result<()> {
        self.get_aliases()?.remove(alias);
        self.save_to_disk()?;
        Ok(())
    }

    pub fn get(&mut self, alias: &str) -> crate::Result<Option<&String>> {
        Ok(self.get_aliases()?.get(alias))
    }
}

impl OpenDictionary {
    pub fn from_alias_with_manager<Options: AsMut<AliasManager>>(
        alias: &str,
        mut options: Options,
    ) -> crate::Result<OpenDictionary> {
        options.as_mut().get(alias)?.map_or_else(
            || Err(crate::Error::AliasNotFound(alias.to_string())),
            OpenDictionary::from_path,
        )
    }

    pub fn alias_to_with_manager<Options: AsMut<AliasManager>>(
        &self,
        name: &str,
        mut options: Options,
    ) -> crate::Result<()> {
        options.as_mut().add(name, self)
    }

    pub fn alias_to(&self, name: &str) -> crate::Result<()> {
        self.alias_to_with_manager(name, AliasManager::default())
    }

    pub fn from_alias(alias: &str) -> crate::Result<OpenDictionary> {
        Self::from_alias_with_manager(alias, AliasManager::default())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use crate::alias::DEFAULT_ALIAS_FILE;

    use super::AliasManager;

    /// Test that custom config directory is properly passed to alias manager by creating an alias
    #[test]
    fn test_custom_config_dir() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().to_path_buf();
        let alias_manager = AliasManager::new(&config_path);

        assert_eq!(alias_manager.path(), config_path.join(DEFAULT_ALIAS_FILE));
    }
}
