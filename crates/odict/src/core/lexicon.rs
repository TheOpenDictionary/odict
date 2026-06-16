//! Lexicon extraction operations for ODict dictionaries.
//!
//! This module provides functionality to extract sorted lists of terms (lexicons)
//! from dictionaries. A lexicon represents all the headwords/terms available
//! in a dictionary, sorted alphabetically.
//!
//! # Overview
//!
//! The lexicon functionality allows you to:
//! - Extract all terms from a dictionary as a sorted list
//! - Get a quick overview of dictionary contents
//! - Generate word lists for analysis or display
//!
//! # Examples
//!
//! ## Extracting a Lexicon from a Dictionary
//!
//! ```rust
//! use odict::Dictionary;
//!
//! let dict = Dictionary::from_path("dictionary.xml")?;
//! let terms = dict.lexicon();
//!
//! // Print all terms in alphabetical order
//! for term in terms {
//!     println!("{}", term);
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
//! let terms = archived.lexicon();
//!
//! println!("Dictionary contains {} terms", terms.len());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::schema::{ArchivedDictionary, Dictionary};

macro_rules! lexicon {
    ($t:ident) => {
        impl $t {
            /// Extract a sorted lexicon (list of terms) from the dictionary.
            ///
            /// This method collects all entry terms from the dictionary and returns
            /// them as a sorted vector of string references. The terms are sorted
            /// alphabetically using standard string ordering.
            ///
            /// # Returns
            ///
            /// A `Vec<&str>` containing all dictionary terms in alphabetical order.
            /// Each term appears exactly once, even if there are multiple entries
            /// with the same term.
            ///
            /// # Examples
            ///
            /// ```rust
            /// use odict::Dictionary;
            ///
            /// let dict = Dictionary::from_path("dictionary.xml")?;
            /// let lexicon = dict.lexicon();
            ///
            /// // Print first 10 terms
            /// for term in lexicon.iter().take(10) {
            ///     println!("{}", term);
            /// }
            /// # Ok::<(), Box<dyn std::error::Error>>(())
            /// ```
            ///
            /// # Performance
            ///
            /// This operation has O(n log n) complexity due to sorting, where n is
            /// the number of entries in the dictionary. The terms are collected
            /// first, then sorted in-place.
            pub fn lexicon(&self) -> Vec<&str> {
                let mut vec: Vec<&str> = self
                    .entries
                    .iter()
                    .map(|entry| entry.term.as_str())
                    .collect();
                vec.sort();
                vec
            }
        }
    };
}

lexicon!(Dictionary);
lexicon!(ArchivedDictionary);
