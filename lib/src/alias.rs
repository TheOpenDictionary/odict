use serde_json::to_vec;
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, read_to_string},
    path::PathBuf,
};

use crate::OpenDictionary;

use super::config::get_config_dir;

#[derive(Debug, Clone)]
pub struct AliasManager {
    path: PathBuf,
    aliases: HashMap<String, String>,
}

impl AsRef<AliasManager> for AliasManager {
    fn as_ref(&self) -> &AliasManager {
        self
    }
}

impl AliasManager {
    pub fn new<S: AsRef<OsStr> + ?Sized>(
        config_path: &S, // May be used in the future?
    ) -> crate::Result<Self> {
        let path = PathBuf::from(config_path);

        if !path.exists() {
            fs::write(&path, "{}")?;
        }

        let config = read_to_string(&path)?;
        let aliases: HashMap<String, String> = serde_json::from_str(&config)?;

        Ok(Self { path, aliases })
    }
}

impl Default for AliasManager {
    fn default() -> Self {
        let config_path = get_config_dir().unwrap().join("aliases.json");
        Self::new(&config_path).unwrap()
    }
}

impl AliasManager {
    fn save_to_disk(&self) -> crate::Result<()> {
        let config_bytes = to_vec(&self.aliases)?;
        fs::write(&self.path, config_bytes)?;
        Ok(())
    }

    pub fn add(&mut self, alias: &str, file: &OpenDictionary) -> crate::Result<()> {
        if self.get(alias).is_none() {
            self.set(alias, file)
        } else {
            Err(crate::Error::AliasExists)
        }
    }

    pub fn set(&mut self, alias: &str, file: &OpenDictionary) -> crate::Result<()> {
        match &file.path() {
            Some(path) => {
                self.aliases
                    .insert(alias.to_string(), path.to_string_lossy().to_string());
                self.save_to_disk()
            }
            None => Err(crate::Error::DictionaryMissingPath),
        }
    }

    pub fn delete(&mut self, alias: &str) -> crate::Result<()> {
        self.aliases.remove(alias);
        self.save_to_disk()?;
        Ok(())
    }

    pub fn get(&self, alias: &str) -> Option<&String> {
        self.aliases.get(alias)
    }
}

#[derive(Debug, Default, Clone)]
pub struct AliasOptions {
    manager: AliasManager,
}

impl AsRef<AliasOptions> for AliasOptions {
    fn as_ref(&self) -> &AliasOptions {
        self
    }
}

impl AsMut<AliasOptions> for AliasOptions {
    fn as_mut(&mut self) -> &mut AliasOptions {
        self
    }
}

impl AliasOptions {
    pub fn with_manager(mut self, manager: AliasManager) -> Self {
        self.manager = manager;
        self
    }
}

impl OpenDictionary {
    pub fn from_alias_with_options<Options: AsRef<AliasOptions>>(
        alias: &str,
        options: Options,
    ) -> crate::Result<OpenDictionary> {
        options.as_ref().manager.get(alias).map_or_else(
            || Err(crate::Error::AliasNotFound(alias.to_string())),
            OpenDictionary::from_path,
        )
    }

    pub fn alias_to_with_options<Options: AsMut<AliasOptions>>(
        &self,
        name: &str,
        mut options: Options,
    ) -> crate::Result<()> {
        options.as_mut().manager.add(name, self)
    }

    pub fn alias_to(&self, name: &str) -> crate::Result<()> {
        self.alias_to_with_options(name, AliasOptions::default())
    }

    pub fn from_alias(alias: &str) -> crate::Result<OpenDictionary> {
        Self::from_alias_with_options(alias, AliasOptions::default())
    }
}
