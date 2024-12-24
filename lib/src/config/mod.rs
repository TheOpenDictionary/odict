mod aliases;
mod config;

pub use aliases::*;
pub use config::*;

use crate::{DictionaryFile, DictionaryReader};

impl DictionaryReader {
    pub fn read_from_path_or_alias(&self, path_or_alias: &str) -> crate::Result<DictionaryFile> {
        self.read_from_path_or_alias_with_manager(path_or_alias, &AliasManager::default())
    }

    pub fn read_from_path_or_alias_with_manager(
        &self,
        path_or_alias: &str,
        manager: &AliasManager,
    ) -> crate::Result<DictionaryFile> {
        match manager.get(path_or_alias) {
            Some(path) => self.read_from_path(path),
            None => self.read_from_path(path_or_alias),
        }
    }
}
