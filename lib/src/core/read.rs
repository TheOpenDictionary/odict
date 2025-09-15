//! Dictionary reading and deserialization operations for ODict.
//!
//! This module provides functionality to read and deserialize dictionaries from
//! various sources, including XML files and binary ODict format files. It handles
//! format validation, version compatibility checking, and decompression.
//!
//! # Overview
//!
//! The reading functionality supports:
//! - Loading dictionaries from XML files
//! - Loading compiled dictionaries from binary ODict files
//! - Reading from file paths or byte arrays
//! - Automatic format detection and validation
//! - Version compatibility verification
//! - Decompression of binary content
//!
//! # Binary Format Structure
//!
//! The ODict binary format consists of:
//! 1. **Signature** (5 bytes): "ODICT" magic bytes for format identification
//! 2. **Version Length** (8 bytes): Length of the version string in little-endian
//! 3. **Version** (variable): UTF-8 encoded semantic version string
//! 4. **Content Length** (8 bytes): Length of compressed content in little-endian
//! 5. **Content** (variable): Compressed serialized dictionary data
//!
//! # Examples
//!
//! ## Loading from XML
//!
//! ```rust
//! use odict::Dictionary;
//!
//! let dict = Dictionary::from_path("dictionary.xml")?;
//! println!("Loaded {} entries", dict.entries.len());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Loading from Binary Format
//!
//! ```rust
//! use odict::OpenDictionary;
//!
//! let dict = OpenDictionary::from_path("dictionary.odict")?;
//! println!("Dictionary version: {}", dict.version);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Loading from Bytes
//!
//! ```rust
//! use odict::OpenDictionary;
//!
//! let bytes = std::fs::read("dictionary.odict")?;
//! let dict = OpenDictionary::from_bytes(&bytes)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::{
    io::{Cursor, Read},
    path::Path,
};

use byteorder::{LittleEndian, ReadBytesExt};

use super::consts::{SIGNATURE, VERSION};
use crate::{
    compress::decompress,
    error::Error,
    fs::{self},
    schema::Dictionary,
    version::SemanticVersion,
    OpenDictionary,
};
use std::str::FromStr;

/* -------------------------------------------------------------------------- */
/*                               Helper Methods                               */
/* -------------------------------------------------------------------------- */

/// Read and validate the ODict signature from a binary stream.
///
/// This function reads the first 5 bytes from the stream and verifies they
/// match the expected "ODICT" signature. This ensures the file is a valid
/// ODict binary format.
///
/// # Arguments
///
/// * `reader` - A cursor over the binary data
///
/// # Returns
///
/// The signature as a string if valid, or an error if invalid.
///
/// # Errors
///
/// Returns [`Error::InvalidSignature`] if the signature doesn't match "ODICT".
fn read_signature<T>(reader: &mut Cursor<T>) -> crate::Result<String>
where
    T: AsRef<[u8]>,
{
    let mut signature_bytes = [0; 5];

    reader.read_exact(&mut signature_bytes)?;

    let signature = signature_bytes.to_vec();

    if signature != SIGNATURE {
        return Err(Error::InvalidSignature);
    }

    Ok(String::from_utf8(signature)?)
}

/// Read and validate the version information from a binary stream.
///
/// This function reads the version length, then the version string, and
/// validates that it's compatible with the current library version.
///
/// # Arguments
///
/// * `reader` - A cursor over the binary data
///
/// # Returns
///
/// The parsed semantic version if compatible, or an error if incompatible.
///
/// # Errors
///
/// Returns [`Error::Incompatible`] if the version is not compatible with
/// the current library version.
fn read_version<T>(reader: &mut Cursor<T>) -> crate::Result<SemanticVersion>
where
    T: AsRef<[u8]>,
{
    let version_len = reader.read_u64::<LittleEndian>()?;
    let mut version_bytes = vec![0; version_len as usize];

    reader.read_exact(&mut version_bytes)?;

    let version = SemanticVersion::from(version_bytes);

    if !version.is_compatible(&VERSION) {
        return Err(Error::Incompatible(
            version.to_string(),
            VERSION.to_string(),
        ));
    }

    Ok(version)
}

/// Read and decompress the dictionary content from a binary stream.
///
/// This function reads the content length, then the compressed content,
/// and decompresses it to obtain the raw serialized dictionary data.
///
/// # Arguments
///
/// * `reader` - A cursor over the binary data
///
/// # Returns
///
/// The decompressed content as a byte vector.
///
/// # Errors
///
/// Returns an error if decompression fails or if the content is corrupted.
fn read_content<T>(reader: &mut Cursor<T>) -> crate::Result<Vec<u8>>
where
    T: AsRef<[u8]>,
{
    let content_size = reader.read_u64::<LittleEndian>()?;
    let mut content_bytes = vec![0; content_size as usize];

    reader.read_exact(&mut content_bytes)?;

    let content = decompress(&content_bytes)?;

    Ok(content)
}

/* -------------------------------------------------------------------------- */
/*                              DictionaryReader                              */
/* -------------------------------------------------------------------------- */

/// A reader for dictionary operations.
///
/// This struct provides a namespace for dictionary reading operations,
/// though most functionality is implemented directly on the dictionary types.
#[derive(Clone, Debug, Default)]
pub struct DictionaryReader {}

impl Dictionary {
    /// Load a dictionary from an XML file.
    ///
    /// This method reads an XML file from the specified path and parses it
    /// into a [`Dictionary`] structure. The XML must conform to the ODict
    /// schema format.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the XML dictionary file
    ///
    /// # Returns
    ///
    /// A [`Dictionary`] instance containing the parsed data.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The XML is malformed or doesn't conform to the ODict schema
    /// - File system permissions prevent access
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::Dictionary;
    ///
    /// let dict = Dictionary::from_path("examples/dictionary.xml")?;
    /// println!("Loaded dictionary with {} entries", dict.entries.len());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let buffer = crate::fs::read_to_string(path)?;
        Self::from_str(&buffer)
    }
}

impl OpenDictionary {
    /// Load a compiled dictionary from binary data.
    ///
    /// This method parses binary data in the ODict format, validating the
    /// signature, checking version compatibility, and decompressing the content.
    /// The resulting [`OpenDictionary`] can be used for fast lookups and operations.
    ///
    /// # Arguments
    ///
    /// * `data` - Binary data in ODict format
    ///
    /// # Returns
    ///
    /// An [`OpenDictionary`] instance ready for use.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The signature is invalid (not an ODict file)
    /// - The version is incompatible with this library
    /// - The content cannot be decompressed
    /// - The binary format is corrupted
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::OpenDictionary;
    ///
    /// let bytes = std::fs::read("dictionary.odict")?;
    /// let dict = OpenDictionary::from_bytes(&bytes)?;
    /// println!("Dictionary version: {}", dict.version);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes<T>(data: T) -> crate::Result<OpenDictionary>
    where
        T: AsRef<[u8]>,
    {
        let mut reader = Cursor::new(data);
        let signature = read_signature(&mut reader)?;
        let version = read_version(&mut reader)?;
        let content = read_content(&mut reader)?;

        Ok(Self {
            signature: signature.clone(),
            version: version.clone(),
            path: None,
            bytes: content,
        })
    }

    /// Load a compiled dictionary from a binary file.
    ///
    /// This method reads a binary ODict file from the specified path and
    /// loads it into an [`OpenDictionary`] instance. The file path is stored
    /// for reference.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the binary ODict file
    ///
    /// # Returns
    ///
    /// An [`OpenDictionary`] instance with the path information preserved.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The file is not a valid ODict binary format
    /// - Version compatibility issues
    /// - File system permissions prevent access
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::OpenDictionary;
    ///
    /// let dict = OpenDictionary::from_path("dictionary.odict")?;
    /// if let Some(path) = &dict.path {
    ///     println!("Loaded from: {}", path.display());
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> crate::Result<OpenDictionary> {
        let buffer = fs::read_to_bytes(&path)?;
        let mut result = Self::from_bytes(&buffer)?;
        result.path = Some(path.as_ref().to_path_buf());
        Ok(result)
    }
}
