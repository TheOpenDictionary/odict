use crate::compress::{compress, CompressOptions};
use crate::error::Error;
use crate::schema::Dictionary;

use super::consts::{SIGNATURE, VERSION};

pub struct CompilerOptions {
    pub compress_options: CompressOptions,
}

impl AsRef<CompilerOptions> for CompilerOptions {
    fn as_ref(&self) -> &CompilerOptions {
        self
    }
}

impl CompilerOptions {
    pub fn compression(mut self, compress_options: CompressOptions) -> Self {
        self.compress_options = compress_options;
        self
    }
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            compress_options: CompressOptions::default(),
        }
    }
}

impl Dictionary {
    pub fn to_bytes(&self) -> crate::Result<Vec<u8>> {
        self.to_bytes_with_options(CompilerOptions::default())
    }

    pub fn to_bytes_with_options<Options: AsRef<CompilerOptions>>(
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
