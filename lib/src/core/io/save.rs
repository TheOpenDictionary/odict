use std::path::Path;
use std::{fs::File, io::Write};

use crate::io::CompileOptions;
use crate::{Dictionary, OpenDictionary};

impl Dictionary {
    pub fn save<P: AsRef<Path>>(&self, path: P) -> crate::Result<OpenDictionary> {
        self.save_with_opts(path, CompileOptions::default())
    }

    pub fn save_with_opts<Options: AsRef<CompileOptions>, P: AsRef<Path>>(
        &self,
        path: P,
        options: Options,
    ) -> crate::Result<OpenDictionary> {
        let buf = self.compile_with_opts(options)?;
        let mut file = File::create(path)?;
        file.write_all(&buf)?;
        OpenDictionary::from_bytes(&buf)
    }
}
