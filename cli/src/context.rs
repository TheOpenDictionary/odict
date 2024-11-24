use console::Term;
use std::io::Write;
use std::sync::LazyLock;

use crate::CLI;
use odict::{config::AliasManager, DictionaryReader, DictionaryWriter};

pub struct CLIContext<'a> {
    pub cli: &'a CLI,
    pub alias_manager: LazyLock<AliasManager>,
    pub reader: LazyLock<DictionaryReader>,
    pub writer: LazyLock<DictionaryWriter>,
    pub stdout: Term,
    pub stderr: Term,
}

impl<'a> CLIContext<'a> {
    pub fn default(cli: &'a CLI) -> Self {
        Self {
            cli,
            alias_manager: LazyLock::new(|| AliasManager::default()),
            reader: LazyLock::new(|| DictionaryReader::default()),
            writer: LazyLock::new(|| DictionaryWriter::default()),
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
