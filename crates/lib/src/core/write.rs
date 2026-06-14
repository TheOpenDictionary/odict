//! Dictionary writing and persistence operations for ODict.
//!
//! This module provides functionality to save compiled dictionaries to disk
//! in the binary ODict format. It handles file creation, binary serialization,
//! and path management for persistent storage.
//!
//! # Overview
//!
//! The writing functionality allows you to:
//! - Save compiled dictionaries to disk
//! - Customize compilation options during save
//! - Automatically update path references
//! - Ensure data integrity through proper file handling
//!
//! # File Format
//!
//! Dictionaries are saved in the binary ODict format, which includes:
//! - Format signature and version information
//! - Compressed serialized dictionary data
//! - Metadata for compatibility checking
//!
//! # Examples
//!
//! ## Basic Dictionary Saving
//!
//! ```rust
//! use odict::{Dictionary, OpenDictionary};
//!
//! let dict = Dictionary::from_path("source.xml")?;
//! let mut compiled = dict.build()?;
//!
//! // Save with default options
//! compiled.to_disk("output.odict")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Saving with Custom Options
//!
//! ```rust
//! use odict::{Dictionary, OpenDictionary, CompilerOptions, CompressOptions};
//!
//! let dict = Dictionary::from_path("source.xml")?;
//! let mut compiled = dict.build()?;
//!
//! let options = CompilerOptions::default()
//!     .with_compression(CompressOptions::default());
//!
//! compiled.to_disk_with_options("output.odict", options)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::fs::canonicalize;
use std::path::Path;
use std::{fs::File, io::Write};

use crate::compile::CompilerOptions;
use crate::OpenDictionary;

impl OpenDictionary {
    /// Save the dictionary to disk using default compilation options.
    ///
    /// This method writes the dictionary to the specified file path in the
    /// binary ODict format. It uses default compression settings and updates
    /// the dictionary's internal path reference to the saved location.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path where the dictionary should be saved
    ///
    /// # Returns
    ///
    /// `Ok(())` if the save operation succeeds.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be created or written to
    /// - Compilation/compression fails
    /// - File system permissions prevent writing
    /// - The path cannot be canonicalized
    ///
    /// # Side Effects
    ///
    /// - Creates or overwrites the file at the specified path
    /// - Updates the dictionary's internal path reference
    /// - Ensures all data is flushed to disk
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::{Dictionary, OpenDictionary};
    ///
    /// let dict = Dictionary::from_path("source.xml")?;
    /// let mut compiled = dict.build()?;
    ///
    /// compiled.to_disk("my_dictionary.odict")?;
    ///
    /// // Path is now updated
    /// if let Some(path) = &compiled.path {
    ///     println!("Saved to: {}", path.display());
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn to_disk<P: AsRef<Path>>(&mut self, path: P) -> crate::Result<()> {
        self.to_disk_with_options(path, CompilerOptions::default())
    }

    /// Save the dictionary to disk with custom compilation options.
    ///
    /// This method provides fine-grained control over the save process,
    /// allowing customization of compression settings and other compilation
    /// options. The dictionary is written in the binary ODict format.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path where the dictionary should be saved
    /// * `options` - Compilation options to customize the save process
    ///
    /// # Returns
    ///
    /// `Ok(())` if the save operation succeeds.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be created or written to
    /// - Compilation fails with the specified options
    /// - Compression fails
    /// - File system permissions prevent writing
    /// - The path cannot be canonicalized
    ///
    /// # Side Effects
    ///
    /// - Creates or overwrites the file at the specified path
    /// - Updates the dictionary's internal path reference to the canonical path
    /// - Ensures all data is properly flushed to disk
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::{Dictionary, OpenDictionary, CompilerOptions, CompressOptions};
    ///
    /// let dict = Dictionary::from_path("source.xml")?;
    /// let mut compiled = dict.build()?;
    ///
    /// // Use custom compression settings
    /// let options = CompilerOptions::default()
    ///     .with_compression(CompressOptions::default());
    ///
    /// compiled.to_disk_with_options("optimized.odict", options)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Performance
    ///
    /// The save operation involves:
    /// 1. Compiling the dictionary to binary format with specified options
    /// 2. Creating/opening the target file
    /// 3. Writing all data to disk
    /// 4. Flushing to ensure data persistence
    /// 5. Canonicalizing the path for accurate reference
    pub fn to_disk_with_options<Options: AsRef<CompilerOptions>, P: AsRef<Path>>(
        &mut self,
        path: P,
        options: Options,
    ) -> crate::Result<()> {
        let buf = self.to_bytes_with_options(options)?;
        let mut file = File::create(&path)?;

        file.write_all(&buf)?;
        file.flush()?;

        self.path = canonicalize(path)?.to_str().map(std::path::PathBuf::from);

        Ok(())
    }
}
