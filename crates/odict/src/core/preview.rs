//! Entry preview generation for ODict dictionaries.
//!
//! This module provides functionality to generate concise text previews of dictionary
//! entries by extracting and concatenating their definitions. Previews are useful for
//! displaying quick summaries of entries without showing the full structured data.
//!
//! # Overview
//!
//! The preview functionality allows you to:
//! - Generate text summaries of dictionary entries
//! - Customize the delimiter used to separate definitions
//! - Handle both regular and grouped definitions
//! - Optionally convert markdown to plain text (when markdown feature is enabled)
//!
//! # Examples
//!
//! ## Basic Preview Generation
//!
//! ```rust
//! use odict::{Dictionary, PreviewOptions};
//!
//! let dict = Dictionary::from_path("dictionary.xml")?;
//! if let Some(entry) = dict.entries.iter().next() {
//!     let preview = entry.preview(PreviewOptions::default());
//!     println!("Preview: {}", preview);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Custom Delimiter
//!
//! ```rust
//! use odict::{Dictionary, PreviewOptions};
//!
//! let dict = Dictionary::from_path("dictionary.xml")?;
//! if let Some(entry) = dict.entries.iter().next() {
//!     let options = PreviewOptions::default().delimiter(" | ".to_string());
//!     let preview = entry.preview(options);
//!     println!("Preview: {}", preview);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#[cfg(feature = "markdown")]
use crate::md::to_text;
use crate::schema::{ArchivedDefinitionType, ArchivedEntry, DefinitionType, Entry};

/// Configuration options for generating entry previews.
///
/// This struct allows customization of how definitions are joined together
/// when creating a preview string from a dictionary entry.
pub struct PreviewOptions {
    delimiter: String,
}

impl Default for PreviewOptions {
    /// Create default preview options.
    ///
    /// The default delimiter is `"; "` (semicolon followed by space), which
    /// provides a natural separation between multiple definitions.
    fn default() -> Self {
        Self {
            delimiter: "; ".to_string(),
        }
    }
}

impl PreviewOptions {
    /// Set a custom delimiter for joining definitions.
    ///
    /// # Arguments
    ///
    /// * `delimiter` - The string to use for separating definitions in the preview
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::PreviewOptions;
    ///
    /// let options = PreviewOptions::default()
    ///     .delimiter(" | ".to_string());
    /// ```
    pub fn delimiter(mut self, delimiter: String) -> Self {
        self.delimiter = delimiter;
        self
    }
}

/// Convert text content to plain text.
///
/// When the markdown feature is disabled, this function returns the input unchanged.
/// When the markdown feature is enabled, it converts markdown to plain text.
#[cfg(not(feature = "markdown"))]
fn to_text(value: &str) -> &str {
    value
}

macro_rules! preview {
    ($t:ident, $d:ident) => {
        impl $t {
            /// Generate a text preview of this dictionary entry.
            ///
            /// This method extracts all definitions from the entry's etymologies and senses,
            /// converts them to plain text (if markdown feature is enabled), and joins them
            /// using the specified delimiter.
            ///
            /// # Arguments
            ///
            /// * `options` - Configuration for preview generation
            ///
            /// # Returns
            ///
            /// A `String` containing all definitions joined by the specified delimiter.
            /// If the entry has no definitions, returns an empty string.
            ///
            /// # Examples
            ///
            /// ```rust
            /// use odict::{Dictionary, PreviewOptions};
            ///
            /// let dict = Dictionary::from_path("dictionary.xml")?;
            /// if let Some(entry) = dict.entries.iter().next() {
            ///     // Use default options ("; " delimiter)
            ///     let preview = entry.preview(PreviewOptions::default());
            ///
            ///     // Use custom delimiter
            ///     let custom_preview = entry.preview(
            ///         PreviewOptions::default().delimiter(" | ".to_string())
            ///     );
            /// }
            /// # Ok::<(), Box<dyn std::error::Error>>(())
            /// ```
            ///
            /// # Processing Order
            ///
            /// Definitions are processed in this order:
            /// 1. Iterate through etymologies
            /// 2. For each etymology, iterate through senses
            /// 3. For each sense, iterate through definitions
            /// 4. Handle both individual definitions and definition groups
            /// 5. Convert markdown to text (if feature enabled)
            /// 6. Join all definitions with the specified delimiter
            pub fn preview(&self, options: PreviewOptions) -> String {
                let definitions: Vec<String> = self
                    .etymologies
                    .iter()
                    .flat_map(|e| -> Vec<String> {
                        e.senses
                            .iter()
                            .flat_map(|s| -> Vec<String> {
                                s.definitions
                                    .iter()
                                    .flat_map(|value| -> Vec<String> {
                                        match value {
                                            $d::Definition(d) => {
                                                vec![to_text(d.value.as_str()).to_string()]
                                            }
                                            $d::Group(g) => g
                                                .definitions
                                                .iter()
                                                .map(|d| to_text(d.value.as_str()).to_string())
                                                .collect(),
                                        }
                                    })
                                    .collect()
                            })
                            .collect()
                    })
                    .collect();

                definitions.join(&options.delimiter)
            }
        }
    };
}

preview!(Entry, DefinitionType);
preview!(ArchivedEntry, ArchivedDefinitionType);
