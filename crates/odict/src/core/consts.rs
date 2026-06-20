//! Core constants for the ODict binary format.
//!
//! This module defines the fundamental constants used throughout the ODict
//! library for binary format identification, versioning, and compatibility
//! checking.
//!
//! # Overview
//!
//! The constants defined here are used for:
//! - Binary format identification through magic signatures
//! - Version tracking and compatibility verification
//! - Ensuring consistent format standards across the library
//!
//! # Binary Format Identification
//!
//! The [`SIGNATURE`] constant provides the magic bytes that identify ODict
//! binary files. This signature is written at the beginning of every compiled
//! dictionary file and verified during reading operations.
//!
//! # Version Management
//!
//! The [`VERSION`] constant contains the current library version, automatically
//! derived from the Cargo package version. This is used for compatibility
//! checking when reading dictionary files created with different library versions.

use std::sync::LazyLock;

use crate::version::SemanticVersion;

/// Magic signature bytes for ODict binary format identification.
///
/// This 5-byte signature ("ODICT") is written at the beginning of every
/// compiled dictionary file to identify it as a valid ODict binary format.
/// The signature is checked during file reading to ensure format validity.
///
/// # Format
///
/// The signature consists of the ASCII bytes for "ODICT":
/// - `O` (0x4F)
/// - `D` (0x44)
/// - `I` (0x49)
/// - `C` (0x43)
/// - `T` (0x54)
///
/// # Usage
///
/// This constant is used internally by the reading and writing operations
/// and should not typically be used directly by library consumers.
pub const SIGNATURE: &[u8] = b"ODICT";

/// Current library version for compatibility checking.
///
/// This constant contains the semantic version of the current library,
/// automatically derived from the Cargo package version at compile time.
/// It's used to ensure compatibility between dictionary files and the
/// library version attempting to read them.
///
/// # Compatibility Rules
///
/// Dictionary files are considered compatible if they have:
/// - The same major version number as the library
/// - The same prerelease status (stable vs. prerelease)
///
/// # Lazy Initialization
///
/// The version is lazily initialized from the `CARGO_PKG_VERSION` environment
/// variable, which is automatically set by Cargo during compilation. This
/// ensures the version always matches the actual package version.
///
/// # Examples
///
/// ```rust
/// use odict::core::consts::VERSION;
///
/// println!("Library version: {}", *VERSION);
/// ```
pub const VERSION: LazyLock<SemanticVersion> =
    LazyLock::new(|| SemanticVersion::from(env!("CARGO_PKG_VERSION")));
