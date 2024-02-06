use std::{error::Error, fs::File, io::Write, path::PathBuf};

use crate::Dictionary;

impl Dictionary {
    pub fn write_to_path(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let bytes = self.compile()?;
        let mut file = File::create(path)?;

        file.write_all(&bytes)?;

        Ok(())
    }
}
