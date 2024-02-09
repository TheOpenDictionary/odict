use std::io::Write;

use crate::CLI;
use odict::{DictionaryReader, DictionaryWriter};

pub struct CLIContext<'a> {
    pub cli: &'a CLI,
    pub reader: DictionaryReader,
    pub writer: DictionaryWriter,
    pub stdout: Box<dyn Write>,
    pub stderr: Box<dyn Write>,
}

impl<'a> CLIContext<'a> {
    pub fn default(cli: &'a CLI) -> Self {
        Self {
            cli,
            reader: DictionaryReader::default(),
            writer: DictionaryWriter::default(),
            stdout: Box::new(std::io::stdout()),
            stderr: Box::new(std::io::stderr()),
        }
    }

    pub fn print(&mut self, msg: String) {
        self.stdout.write_all(msg.as_bytes()).unwrap();
    }
}
