use std::path::Path;
use std::{fs::File, io::Write};

use crate::compile::CompilerOptions;
use crate::{schema::Dictionary, OpenDictionary};

impl Dictionary {
    pub fn to_disk<P: AsRef<Path>>(&self, path: P) -> crate::Result<OpenDictionary> {
        self.to_disk_with_options(path, CompilerOptions::default())
    }

    pub fn to_disk_with_options<Options: AsRef<CompilerOptions>, P: AsRef<Path>>(
        &self,
        path: P,
        options: Options,
    ) -> crate::Result<OpenDictionary> {
        let buf = self.to_bytes_with_options(options)?;
        let mut file = File::create(path)?;
        file.write_all(&buf)?;
        OpenDictionary::from_bytes(&buf)
    }
}
