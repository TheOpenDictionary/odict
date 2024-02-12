mod aliases;
mod config;

use std::error::Error;

pub use aliases::*;

use crate::{DictionaryFile, DictionaryReader};

impl DictionaryReader {
    pub fn read_from_path_or_alias(
        &self,
        path_or_alias: &str,
    ) -> Result<DictionaryFile, Box<dyn Error>> {
        self.read_from_path_or_alias_with_manager(path_or_alias, &AliasManager::default())
    }

    pub fn read_from_path_or_alias_with_manager(
        &self,
        path_or_alias: &str,
        manager: &AliasManager,
    ) -> Result<DictionaryFile, Box<dyn Error>> {
        match manager.get(path_or_alias) {
            Some(path) => self.read_from_path(path),
            None => self.read_from_path(path_or_alias),
        }
    }
}
