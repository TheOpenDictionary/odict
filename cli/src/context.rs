use std::io::Write;

use console::Term;

use crate::CLI;

pub struct CLIContext<'a> {
    pub cli: &'a CLI,
    pub stdout: Term,
    pub stderr: Term,
}

impl<'a> CLIContext<'a> {
    pub fn default(cli: &'a CLI) -> Self {
        Self {
            cli,
            stdout: Term::stdout(),
            stderr: Term::stderr(),
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
