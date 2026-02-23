//! Entry resolution operations for ODict dictionaries.
//!
//! This module provides functionality to resolve (look up) dictionary entries
//! by their exact term. Resolution is a simple, direct lookup operation that
//! returns the entry if it exists, or None if not found.
//!
//! # Overview
//!
//! The resolve functionality allows you to:
//! - Look up entries by exact term match
//! - Get direct access to entry data structures
//! - Perform fast O(1) lookups using the underlying hash map
//!
//! # Difference from Lookup
//!
//! Resolution differs from the more complex lookup operations in that it:
//! - Only performs exact matches (no fuzzy matching or strategies)
//! - Does not follow redirects or see_also links
//! - Does not support case-insensitive fallback
//! - Returns the raw entry structure rather than wrapped results
//!
//! # Examples
//!
//! ## Basic Entry Resolution
//!
//! ```rust
//! use odict::Dictionary;
//!
//! let dict = Dictionary::from_path("dictionary.xml")?;
//!
//! if let Some(entry) = dict.resolve("hello") {
//!     println!("Found entry for 'hello': {}", entry.term);
//! } else {
//!     println!("No entry found for 'hello'");
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Working with Archived Dictionaries
//!
//! ```rust
//! use odict::OpenDictionary;
//!
//! let dict = OpenDictionary::from_path("dictionary.odict")?;
//! let archived = dict.contents()?;
//!
//! if let Some(entry) = archived.resolve("world") {
//!     println!("Found archived entry: {}", entry.term.as_str());
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::schema::{ArchivedDictionary, ArchivedEntry, Dictionary, Entry};

macro_rules! resolve {
    ($t:ident, $ret:ident) => {
        impl $t {
            /// Resolve a dictionary entry by exact term match.
            ///
            /// This method performs a direct lookup in the dictionary's entry collection
            /// using the provided term as the key. The lookup is case-sensitive and
            /// requires an exact match.
            ///
            /// # Arguments
            ///
            /// * `term` - The exact term to look up in the dictionary
            ///
            /// # Returns
            ///
            /// - `Some(&Entry)` - A reference to the entry if found
            /// - `None` - If no entry exists with the exact term
            ///
            /// # Examples
            ///
            /// ```rust
            /// use odict::Dictionary;
            ///
            /// let dict = Dictionary::from_path("dictionary.xml")?;
            ///
            /// // Exact match lookup
            /// if let Some(entry) = dict.resolve("hello") {
            ///     println!("Term: {}", entry.term);
            ///     println!("Etymologies: {}", entry.etymologies.len());
            /// }
            ///
            /// // Case-sensitive - this might not match if entry is "Hello"
            /// let result = dict.resolve("Hello");
            /// # Ok::<(), Box<dyn std::error::Error>>(())
            /// ```
            ///
            /// # Performance
            ///
            /// This operation has O(1) average time complexity as it uses the underlying
            /// hash map for direct key lookup. In the worst case (hash collisions), it
            /// may degrade to O(n) but this is rare in practice.
            ///
            /// # See Also
            ///
            /// For more advanced lookup operations with fuzzy matching, case-insensitive
            /// search, and redirect following, see the [`lookup`](crate::core::lookup) module.
            pub fn resolve<'a>(&'a self, term: &str) -> Option<&'a $ret> {
                self.entries.get(term)
            }
        }
    };
}

resolve!(Dictionary, Entry);
resolve!(ArchivedDictionary, ArchivedEntry);
