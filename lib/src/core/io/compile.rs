use crate::compress::{compress, CompressOptions};
use crate::error::Error;
use crate::Dictionary;

use super::consts::{SIGNATURE, VERSION};

pub struct CompileOptions {
    pub compress_options: CompressOptions,
}

impl AsRef<CompileOptions> for CompileOptions {
    fn as_ref(&self) -> &CompileOptions {
        self
    }
}

impl CompileOptions {
    pub fn compression(mut self, compress_options: CompressOptions) -> Self {
        self.compress_options = compress_options;
        self
    }
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            compress_options: CompressOptions::default(),
        }
    }
}

impl Dictionary {
    pub fn compile(&self) -> crate::Result<Vec<u8>> {
        self.compile_with_opts(CompileOptions::default())
    }

    pub fn compile_with_opts<Options: AsRef<CompileOptions>>(
        &self,
        options: Options,
    ) -> crate::Result<Vec<u8>> {
        let buf = &self.serialize()?;

        let compressed = compress(buf, &options.as_ref().compress_options)
            .map_err(|e| Error::Write(e.to_string()))?;

        let version_bytes = VERSION.as_bytes();
        let version_size = version_bytes.len() as u64;
        let version_size_bytes = version_size.to_le_bytes();

        let compressed_size = compressed.len() as u64;
        let compressed_size_bytes = compressed_size.to_le_bytes();

        let total_size = SIGNATURE.len()
            + version_size_bytes.len()
            + compressed_size_bytes.len()
            + compressed.len();

        if SIGNATURE.len() != 5 {
            return Err(Error::Write("Signature bytes do not equal 5".into()));
        }

        if version_size_bytes.len() != 8 {
            return Err(Error::Write("Version byte count does not equal 8".into()));
        }

        if version_bytes.len() != version_size as usize {
            return Err(Error::Write(
                "Version byte count does not equal the computed byte count".into(),
            ));
        }

        if compressed_size_bytes.len() != 8 {
            return Err(Error::Write(
                "Compressed byte count does not equal 8".into(),
            ));
        }

        if compressed.len() != compressed_size as usize {
            return Err(Error::Write(
                "Compressed byte count does not equal the computed byte count".into(),
            ));
        }

        let mut output = Vec::with_capacity(total_size);

        output.extend_from_slice(SIGNATURE);
        output.extend_from_slice(&version_size_bytes);
        output.extend_from_slice(&version_bytes);
        output.extend_from_slice(&compressed_size_bytes);
        output.extend_from_slice(&compressed);

        return Ok(output);
    }
}
