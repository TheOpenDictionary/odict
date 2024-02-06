use std::error::Error;

use rkyv::to_bytes;

use crate::{models::Dictionary, utils::compress};

use super::version::FILE_VERSION;

const SIGNATURE: &[u8] = b"ODICT";

impl Dictionary {
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let bytes = to_bytes::<_, 4096>(self)?;
        Ok(bytes.to_vec())
    }

    pub fn compile(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let version_bytes = FILE_VERSION.to_le_bytes();

        let compressed = compress(&self.serialize()?)?;
        let compressed_size = compressed.len() as u64;
        let compressed_size_bytes = compressed_size.to_le_bytes();

        let total_size =
            SIGNATURE.len() + version_bytes.len() + compressed_size_bytes.len() + compressed.len();

        let mut output = Vec::with_capacity(total_size);

        output.extend_from_slice(SIGNATURE);
        output.extend_from_slice(&version_bytes);
        output.extend_from_slice(&compressed_size_bytes);
        output.extend_from_slice(&compressed);

        if SIGNATURE.len() != 5 {
            return Err("Signature bytes do not equal 5".into());
        }

        if version_bytes.len() != 2 {
            return Err("Version bytes do not equal 2".into());
        }

        if compressed_size_bytes.len() != 8 {
            return Err("Content byte count does not equal 8".into());
        }

        if compressed.len() != compressed_size as usize {
            return Err("Content does not equal the computed byte count".into());
        }

        Ok(output)
    }
}
