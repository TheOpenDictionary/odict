//! Core functionality for the ODict dictionary format.
//!
//! This module provides the fundamental operations for working with ODict dictionaries,
//! including compilation, reading, writing, lookup, and various utility functions.
//!
//! # Overview
//!
//! The core module is organized into several key areas:
//!
//! - **Compilation & Serialization**: [`compile`] - Convert dictionaries to binary format
//! - **Reading & Deserialization**: [`read`] - Load dictionaries from various sources
//! - **Writing**: [`write`] - Save dictionaries to disk
//! - **Lookup Operations**: [`lookup`] - Search and retrieve dictionary entries
//! - **Dictionary Management**: [`merge`], [`lexicon`] - Combine dictionaries and extract terms
//! - **Utilities**: [`preview`], [`rank`], [`resolve`] - Additional dictionary operations
//! - **Version Management**: [`version`] - Semantic versioning support
//!
//! # Examples
//!
//! ## Basic Dictionary Operations
//!
//! ```rust
//! use odict::{Dictionary, OpenDictionary};
//!
//! // Load a dictionary from XML
//! let dict = Dictionary::from_path("dictionary.xml")?;
//!
//! // Compile to binary format
//! let compiled = dict.build()?;
//!
//! // Save to disk
//! compiled.to_disk("dictionary.odict")?;
//!
//! // Load from binary
//! let loaded = OpenDictionary::from_path("dictionary.odict")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Dictionary Lookup
//!
//! ```rust
//! use odict::{OpenDictionary, LookupOptions, LookupStrategy};
//!
//! let dict = OpenDictionary::from_path("dictionary.odict")?;
//! let archived = dict.contents()?;
//!
//! // Simple lookup
//! let queries = vec!["hello"];
//! let results = archived.lookup(&queries, LookupOptions::default())?;
//!
//! // Advanced lookup with options
//! let options = LookupOptions::default()
//!     .insensitive(true)
//!     .follow(5)
//!     .strategy(LookupStrategy::Split(2));
//! let results = archived.lookup(&queries, options)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod consts;

pub mod compile;
pub mod lexicon;
pub mod lookup;
pub mod merge;
pub mod preview;
pub mod rank;
pub mod read;
pub mod resolve;
pub mod version;
pub mod write;

pub use rkyv::option::ArchivedOption;
