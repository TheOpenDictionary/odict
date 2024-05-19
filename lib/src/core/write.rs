use std::fs::read_to_string;
use std::path::Path;
use std::{error::Error, fs::File, io::Write};

use crate::lz4::compress;
use crate::{Dictionary, ToDictionary};

use super::constants::*;

pub struct DictionaryWriter {}

impl Default for DictionaryWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl DictionaryWriter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write_to_bytes(&self, dictionary: &Dictionary) -> Result<Vec<u8>, Box<dyn Error>> {
        let compressed = compress(&dictionary.serialize()?)?;

        let version_bytes = VERSION.as_bytes();
        let version_size = version_bytes.len() as u64;
        let version_size_bytes = version_size.to_le_bytes();

        let compressed_size = compressed.len() as u64;
        let compressed_size_bytes = compressed_size.to_le_bytes();

        let total_size = SIGNATURE.len()
            + version_size_bytes.len()
            + compressed_size_bytes.len()
            + compressed.len();

        let mut output = Vec::with_capacity(total_size);

        output.extend_from_slice(SIGNATURE);
        output.extend_from_slice(&version_size_bytes);
        output.extend_from_slice(&version_bytes);
        output.extend_from_slice(&compressed_size_bytes);
        output.extend_from_slice(&compressed);

        if SIGNATURE.len() != 5 {
            return Err("Signature bytes do not equal 5".into());
        }

        if version_size_bytes.len() != 8 {
            return Err("Version byte count does not equal 8".into());
        }

        if version_bytes.len() != version_size as usize {
            return Err("Version does not equal the computed byte count".into());
        }

        if compressed_size_bytes.len() != 8 {
            return Err("Content byte count does not equal 8".into());
        }

        if compressed.len() != compressed_size as usize {
            return Err("Content does not equal the computed byte count".into());
        }

        Ok(output)
    }

    pub fn write_to_path<P: AsRef<Path>>(
        &self,
        dictionary: &Dictionary,
        path: P,
    ) -> Result<(), Box<dyn Error>> {
        let bytes = self.write_to_bytes(dictionary)?;
        let mut file = File::create(path)?;

        file.write_all(&bytes)?;

        Ok(())
    }

    /// Compiles an XML file to an ODict dictionary file
    ///
    /// # Arguments
    ///
    /// * `input_path` - The path to the XML file
    /// * `output_path` - The path to the output dictionary file
    ///
    /// # Example
    ///
    /// ```
    /// use odict::DictionaryWriter;
    /// use std::path::PathBuf;
    ///
    /// let writer = DictionaryWriter::new();
    ///
    /// writer.compile_xml("path/to/input.xml", "path/to/output.odict");
    /// ```
    ///
    pub fn compile_xml<I: AsRef<Path>, O: AsRef<Path>>(
        &self,
        input_path: I,
        output_path: O,
    ) -> Result<(), Box<dyn Error>> {
        let dict = read_to_string(input_path)?.to_dictionary()?;
        self.write_to_path(&dict, output_path)?;
        Ok(())
    }
}
