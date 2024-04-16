use console::Term;
use once_cell::sync::Lazy;
use std::io::Write;

use crate::CLI;
use odict::{config::AliasManager, DictionaryReader, DictionaryWriter};

pub struct CLIContext<'a> {
    pub cli: &'a CLI,
    pub alias_manager: Lazy<AliasManager>,
    pub reader: Lazy<DictionaryReader>,
    pub writer: Lazy<DictionaryWriter>,
    pub stdout: Term,
    pub stderr: Term,
}

impl<'a> CLIContext<'a> {
    pub fn default(cli: &'a CLI) -> Self {
        Self {
            cli,
            alias_manager: Lazy::new(|| AliasManager::default()),
            reader: Lazy::new(|| DictionaryReader::default()),
            writer: Lazy::new(|| DictionaryWriter::default()),
            stdout: Term::buffered_stdout(),
            stderr: Term::buffered_stdout(),
        }
    }

    pub fn println<S>(&mut self, msg: S)
    where
        S: AsRef<str>,
    {
        self.stdout
            .write_all(format!("{}\n", msg.as_ref()).as_bytes())
            .unwrap();

        self.stdout.flush().unwrap();
    }
}
