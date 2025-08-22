use std::io::Write;

use console::Term;

use crate::CLI;
use odict::DictionaryWriter;

pub struct CLIContext<'a> {
    pub cli: &'a CLI,

    pub writer: DictionaryWriter,
    pub stdout: Term,
    pub stderr: Term,
}

impl<'a> CLIContext<'a> {
    pub fn default(cli: &'a CLI) -> Self {
        Self {
            cli,
            writer: DictionaryWriter::default(),
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
