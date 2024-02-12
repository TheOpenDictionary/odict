use std::io::Write;

use crate::CLI;
use console::Term;
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
            stdout: Box::new(Term::stdout()),
            stderr: Box::new(Term::stderr()),
        }
    }

    pub fn println(&mut self, msg: String) {
        self.stdout
            .write_all(format!("{}\n", msg).as_bytes())
            .unwrap();
    }

    pub fn print(&mut self, msg: String) {
        self.stdout
            .write_all(format!("{}", msg).as_bytes())
            .unwrap();
    }

    pub fn newln(&mut self) {
        self.stdout.write_all("\n".as_bytes()).unwrap();
    }
}
