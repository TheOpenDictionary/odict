use std::fs::read_to_string;
use std::path::Path;
use std::{fs::File, io::Write};

use crate::compress::{compress, CompressOptions};
use crate::error::Error;
use crate::{Dictionary, ToDictionary};

use super::consts::{SIGNATURE, VERSION};

#[derive(Debug, Clone)]
pub struct DictionaryWriter {}

impl Default for DictionaryWriter {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DictionaryWriterOptions {
    pub compress_options: CompressOptions,
}

impl AsRef<DictionaryWriterOptions> for DictionaryWriterOptions {
    fn as_ref(&self) -> &DictionaryWriterOptions {
        self
    }
}

impl DictionaryWriterOptions {
    pub fn compression(mut self, compress_options: CompressOptions) -> Self {
        self.compress_options = compress_options;
        self
    }
}

impl Default for DictionaryWriterOptions {
    fn default() -> Self {
        Self {
            compress_options: CompressOptions::default(),
        }
    }
}

impl DictionaryWriter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write_to_bytes(&self, dictionary: &Dictionary) -> crate::Result<Vec<u8>> {
        self.write_to_bytes_with_opts(dictionary, DictionaryWriterOptions::default())
    }

    pub fn write_to_bytes_with_opts<Options: AsRef<DictionaryWriterOptions>>(
        &self,
        dictionary: &Dictionary,
        options: Options,
    ) -> crate::Result<Vec<u8>> {
        let compressed = compress(&dictionary.serialize()?, &options.as_ref().compress_options)
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

        let mut output = Vec::with_capacity(total_size);

        output.extend_from_slice(SIGNATURE);
        output.extend_from_slice(&version_size_bytes);
        output.extend_from_slice(&version_bytes);
        output.extend_from_slice(&compressed_size_bytes);
        output.extend_from_slice(&compressed);

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

        Ok(output)
    }

    pub fn write_to_path_with_opts<P: AsRef<Path>, O: AsRef<DictionaryWriterOptions>>(
        &self,
        dictionary: &Dictionary,
        path: P,
        options: O,
    ) -> crate::Result<()> {
        let bytes = self.write_to_bytes_with_opts(dictionary, options)?;
        let mut file = File::create(path)?;

        file.write_all(&bytes)?;

        Ok(())
    }

    pub fn write_to_path<P: AsRef<Path>>(
        &self,
        dictionary: &Dictionary,
        path: P,
    ) -> crate::Result<()> {
        self.write_to_path_with_opts(dictionary, path, DictionaryWriterOptions::default())
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
    ) -> crate::Result<()> {
        self.compile_xml_with_opts(input_path, output_path, DictionaryWriterOptions::default())
    }

    pub fn compile_xml_with_opts<
        I: AsRef<Path>,
        O: AsRef<Path>,
        Options: AsRef<DictionaryWriterOptions>,
    >(
        &self,
        input_path: I,
        output_path: O,
        options: Options,
    ) -> crate::Result<()> {
        let dict = read_to_string(input_path)?.to_dictionary()?;
        self.write_to_path_with_opts(&dict, output_path, options)?;
        Ok(())
    }
}
