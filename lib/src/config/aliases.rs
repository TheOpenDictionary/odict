use std::{error::Error, ffi::OsStr};

use serde_json::to_vec;

use crate::DictionaryFile;

use super::config::{get_config, ODictConfig};

use std::fs;

pub struct AliasManager {
    config: ODictConfig,
}

impl AliasManager {
    // May be used in the future?
    pub fn new<S: AsRef<OsStr> + ?Sized>(config_path: &S) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            config: get_config(Some(config_path))?,
        })
    }
}

impl Default for AliasManager {
    fn default() -> Self {
        Self {
            config: get_config::<String>(None).unwrap(),
        }
    }
}

impl AliasManager {
    fn save_to_disk(&self) -> Result<(), Box<dyn Error>> {
        let config_bytes = to_vec(&self.config.aliases)?;
        fs::write(&self.config.path, config_bytes)?;
        Ok(())
    }

    pub fn add(&mut self, alias: &str, file: &DictionaryFile) -> Result<(), Box<dyn Error>> {
        if self.get(alias).is_none() {
            self.set(alias, file)
        } else {
            Err("An alias with this name already exists!".into())
        }
    }

    pub fn set(&mut self, alias: &str, file: &DictionaryFile) -> Result<(), Box<dyn Error>> {
        match &file.path {
            Some(path) => {
                self.config
                    .aliases
                    .insert(alias.to_string(), path.to_string_lossy().to_string());
                self.save_to_disk()
            }
            None => Err("This dictionary has no path!".into()),
        }
    }

    pub fn delete(&mut self, alias: &str) -> Result<(), Box<dyn Error>> {
        self.config.aliases.remove(alias);
        self.save_to_disk()?;
        Ok(())
    }

    pub fn get(&self, alias: &str) -> Option<&String> {
        self.config.aliases.get(alias)
    }
}

impl DictionaryFile {
    pub fn alias_to(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let mut manager = AliasManager::default();
        manager.add(name, self)
    }
}
