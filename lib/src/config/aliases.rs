use std::{error::Error, ffi::OsStr};

use serde_json::to_vec;

use crate::DictionaryFile;

use super::config::{get_config, ODictConfig};

use std::fs;

pub struct AliasManager {
    config: ODictConfig,
}

impl AliasManager {
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

    pub fn create(&mut self, alias: &str, file: &DictionaryFile) -> Result<(), Box<dyn Error>> {
        match &file.path {
            Some(path) => {
                self.config
                    .aliases
                    .insert(name.to_string(), path.to_string_lossy().to_string());

                self.save_to_disk()?;

                Ok(())
            }
            None => Err("This dictionary has no path!".into()),
        }
    }

    pub fn set(&mut self, alias: &str, file: &DictionaryFile) -> Result<(), Box<dyn Error>> {
        match &file.path {
            Some(path) => {
                self.config
                    .aliases
                    .insert(name.to_string(), path.to_string_lossy().to_string());

                self.save_to_disk()?;

                Ok(())
            }
            None => Err("This dictionary has no path!".into()),
        }
    }

    pub fn delete(&mut self, alias: &str, file: &DictionaryFile) -> Result<(), Box<dyn Error>> {
        match &file.path {
            Some(path) => {
                self.config.aliases.remove(alias.to_string());

                self.save_to_disk()?;

                Ok(())
            }
            None => Err("This dictionary has no path!".into()),
        }
    }
}
