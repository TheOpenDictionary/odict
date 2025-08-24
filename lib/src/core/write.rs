use std::fs::canonicalize;
use std::path::Path;
use std::{fs::File, io::Write};

use crate::compile::CompilerOptions;
use crate::OpenDictionary;

impl OpenDictionary {
    pub fn to_disk<P: AsRef<Path>>(&mut self, path: P) -> crate::Result<()> {
        self.to_disk_with_options(path, CompilerOptions::default())
    }

    pub fn to_disk_with_options<Options: AsRef<CompilerOptions>, P: AsRef<Path>>(
        &mut self,
        path: P,
        options: Options,
    ) -> crate::Result<()> {
        let buf = self.to_bytes_with_options(options)?;
        let mut file = File::create(&path)?;

        file.write_all(&buf)?;
        file.flush()?;

        self.path = canonicalize(path)?
            .to_str()
            .map(|s| std::path::PathBuf::from(s));

        Ok(())
    }
}
