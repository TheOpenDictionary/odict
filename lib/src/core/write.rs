use std::fs::read_to_string;
use std::path::Path;
use std::{error::Error, fs::File, io::Write};

use crate::utils::compress;
use crate::{Dictionary, ToDictionary};

use super::constants::*;

pub struct DictionaryWriter {}

impl DictionaryWriter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn default() -> Self {
        Self {}
    }

    pub fn write_to_bytes(&self, dictionary: &Dictionary) -> Result<Vec<u8>, Box<dyn Error>> {
        let version_bytes = FILE_VERSION.to_le_bytes();

        let compressed = compress(&dictionary.serialize()?)?;
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
    pub fn compile_xml<P: AsRef<Path>>(
        &self,
        input_path: P,
        output_path: P,
    ) -> Result<(), Box<dyn Error>> {
        let dict = read_to_string(input_path)?.to_dictionary()?;
        self.write_to_path(&dict, output_path)?;
        Ok(())
    }
}
