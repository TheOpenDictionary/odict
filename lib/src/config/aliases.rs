use serde_json::to_vec;
use std::{collections::HashMap, error::Error, ffi::OsStr, fs, fs::read_to_string, path::PathBuf};

use crate::DictionaryFile;

use super::config::get_config_dir;

pub struct AliasManager {
    path: PathBuf,
    aliases: HashMap<String, String>,
}

impl AliasManager {
    pub fn new<S: AsRef<OsStr> + ?Sized>(
        config_path: &S, // May be used in the future?
    ) -> Result<Self, Box<dyn Error>> {
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
    fn save_to_disk(&self) -> Result<(), Box<dyn Error>> {
        let config_bytes = to_vec(&self.aliases)?;
        fs::write(&self.path, config_bytes)?;
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
                self.aliases
                    .insert(alias.to_string(), path.to_string_lossy().to_string());
                self.save_to_disk()
            }
            None => Err("This dictionary has no path!".into()),
        }
    }

    pub fn delete(&mut self, alias: &str) -> Result<(), Box<dyn Error>> {
        self.aliases.remove(alias);
        self.save_to_disk()?;
        Ok(())
    }

    pub fn get(&self, alias: &str) -> Option<&String> {
        self.aliases.get(alias)
    }
}

impl DictionaryFile {
    pub fn alias_to(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let mut manager = AliasManager::default();
        manager.add(name, self)
    }
}
