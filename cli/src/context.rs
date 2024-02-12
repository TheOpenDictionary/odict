use once_cell::sync::Lazy;
use std::io::Write;

use crate::CLI;
use odict::{config::AliasManager, DictionaryReader, DictionaryWriter};

pub struct CLIContext<'a> {
    pub cli: &'a CLI,
    pub alias_manager: Lazy<AliasManager>,
    pub reader: Lazy<DictionaryReader>,
    pub writer: Lazy<DictionaryWriter>,
    pub stdout: Box<dyn Write>,
    pub stderr: Box<dyn Write>,
}

impl<'a> CLIContext<'a> {
    pub fn default(cli: &'a CLI) -> Self {
        Self {
            cli,
            alias_manager: Lazy::new(|| AliasManager::default()),
            reader: Lazy::new(|| DictionaryReader::default()),
            writer: Lazy::new(|| DictionaryWriter::default()),
            stdout: Box::new(std::io::stdout()),
            stderr: Box::new(std::io::stderr()),
        }
    }

    pub fn println(&mut self, msg: String) {
        self.stdout
            .write_all(format!("{}\n", msg).as_bytes())
            .unwrap();
    }
}
