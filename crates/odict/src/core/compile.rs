//! Dictionary compilation and binary serialization.
//!
//! This module provides functionality to compile dictionary data structures into
//! the ODict binary format. The compilation process involves serialization,
//! compression, and packaging with metadata headers.
//!
//! # Binary Format Structure
//!
//! The ODict binary format consists of:
//! 1. **Signature** (5 bytes): "ODICT" magic bytes
//! 2. **Version Length** (8 bytes): Length of version string
//! 3. **Version** (variable): Semantic version string
//! 4. **Content Length** (8 bytes): Length of compressed content
//! 5. **Content** (variable): Compressed serialized dictionary data
//!
//! # Examples
//!
//! ## Basic Compilation
//!
//! ```rust
//! use odict::{Dictionary, CompilerOptions};
//!
//! let dict = Dictionary::from_path("dictionary.xml")?;
//! let compiled = dict.build()?;
//! let bytes = compiled.to_bytes()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Compilation with Custom Compression
//!
//! ```rust
//! use odict::{Dictionary, CompilerOptions, CompressOptions};
//!
//! let dict = Dictionary::from_path("dictionary.xml")?;
//! let compiled = dict.build()?;
//!
//! let options = CompilerOptions::default()
//!     .with_compression(CompressOptions::default());
//! let bytes = compiled.to_bytes_with_options(options)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::compress::{compress, CompressOptions};
use crate::error::Error;
use crate::schema::Dictionary;
use crate::OpenDictionary;

use super::consts::{SIGNATURE, VERSION};

/// Configuration options for dictionary compilation.
///
/// This struct allows customization of the compilation process, particularly
/// compression settings that affect the final binary size and performance.
#[derive(Default)]
pub struct CompilerOptions {
    /// Compression options to use during compilation.
    pub compress_options: CompressOptions,
}

impl AsRef<CompilerOptions> for CompilerOptions {
    fn as_ref(&self) -> &CompilerOptions {
        self
    }
}

impl CompilerOptions {
    /// Set custom compression options for the compilation process.
    ///
    /// # Arguments
    ///
    /// * `compress_options` - The compression configuration to use
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::{CompilerOptions, CompressOptions};
    ///
    /// let options = CompilerOptions::default()
    ///     .with_compression(CompressOptions::default());
    /// ```
    pub fn with_compression(mut self, compress_options: CompressOptions) -> Self {
        self.compress_options = compress_options;
        self
    }
}

impl OpenDictionary {
    /// Convert the dictionary to binary format using default compilation options.
    ///
    /// This method serializes the dictionary into the ODict binary format,
    /// applying default compression and packaging it with the appropriate headers.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the complete binary representation of the dictionary.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Compression fails
    /// - Serialization fails
    /// - Binary format validation fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::{Dictionary, OpenDictionary};
    ///
    /// let dict = Dictionary::from_path("dictionary.xml")?;
    /// let compiled = dict.build()?;
    /// let bytes = compiled.to_bytes()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn to_bytes(&self) -> crate::Result<Vec<u8>> {
        self.to_bytes_with_options(CompilerOptions::default())
    }

    /// Convert the dictionary to binary format with custom compilation options.
    ///
    /// This method provides fine-grained control over the compilation process,
    /// allowing customization of compression settings and other options.
    ///
    /// # Arguments
    ///
    /// * `options` - Compilation options to customize the process
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the complete binary representation of the dictionary.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Compression fails with the specified options
    /// - Serialization fails
    /// - Binary format validation fails
    /// - Header construction fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::{Dictionary, OpenDictionary, CompilerOptions, CompressOptions};
    ///
    /// let dict = Dictionary::from_path("dictionary.xml")?;
    /// let compiled = dict.build()?;
    ///
    /// let options = CompilerOptions::default()
    ///     .with_compression(CompressOptions::default());
    /// let bytes = compiled.to_bytes_with_options(options)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn to_bytes_with_options<Options: AsRef<CompilerOptions>>(
        &self,
        options: Options,
    ) -> crate::Result<Vec<u8>> {
        let compressed = compress(&self.bytes, &options.as_ref().compress_options)
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

        Ok(output)
    }
}

impl Dictionary {
    /// Build a compiled dictionary from the current dictionary data.
    ///
    /// This method transforms a [`Dictionary`] into an [`OpenDictionary`] by
    /// serializing the dictionary data and preparing it for binary compilation.
    /// The resulting [`OpenDictionary`] can then be converted to bytes or saved to disk.
    ///
    /// # Returns
    ///
    /// An [`OpenDictionary`] containing the serialized dictionary data with
    /// appropriate metadata (signature, version, etc.).
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Dictionary serialization fails
    /// - Memory allocation fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::Dictionary;
    ///
    /// let dict = Dictionary::from_path("dictionary.xml")?;
    /// let compiled = dict.build()?;
    ///
    /// // Now you can save to disk or convert to bytes
    /// compiled.to_disk("output.odict")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn build(&self) -> crate::Result<OpenDictionary> {
        let dict = OpenDictionary {
            signature: String::from_utf8_lossy(SIGNATURE).to_string(),
            version: VERSION.clone(),
            path: None,
            bytes: self.serialize()?,
        };
        Ok(dict)
    }
}
