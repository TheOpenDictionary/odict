//! Entry ranking operations for ODict dictionaries.
//!
//! This module provides functionality to analyze and extract ranking information
//! from dictionary entries. Rankings are typically used to indicate word frequency,
//! importance, or usage patterns within a dictionary.
//!
//! # Overview
//!
//! The ranking functionality allows you to:
//! - Find the minimum rank across all entries
//! - Find the maximum rank across all entries
//! - Analyze ranking distribution in dictionaries
//!
//! # Ranking System
//!
//! Rankings are optional numeric values associated with dictionary entries.
//! Lower numbers typically indicate higher frequency or importance (e.g., rank 1
//! might be the most common word). Not all entries are required to have ranks.
//!
//! # Examples
//!
//! ## Finding Rank Range
//!
//! ```rust
//! use odict::Dictionary;
//!
//! let dict = Dictionary::from_path("dictionary.xml")?;
//!
//! if let Some(min) = dict.min_rank() {
//!     println!("Highest priority rank: {}", min);
//! }
//!
//! if let Some(max) = dict.max_rank() {
//!     println!("Lowest priority rank: {}", max);
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
//! match (archived.min_rank(), archived.max_rank()) {
//!     (Some(min), Some(max)) => {
//!         println!("Rank range: {} to {}", min, max);
//!     }
//!     _ => println!("No ranked entries found"),
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::schema::{ArchivedDictionary, Dictionary};

impl ArchivedDictionary {
    /// Create an iterator over all rank values in the archived dictionary.
    ///
    /// This internal method filters entries to only those with rank values,
    /// converting archived rank values to native u32 format.
    fn rank_iter(&self) -> impl Iterator<Item = u32> + '_ {
        self.entries
            .iter()
            .filter_map(|entry| entry.rank.as_ref().map(|r| r.to_native()))
    }
}

impl Dictionary {
    /// Create an iterator over all rank values in the dictionary.
    ///
    /// This internal method filters entries to only those with rank values.
    fn rank_iter(&self) -> impl Iterator<Item = u32> + '_ {
        self.entries.iter().filter_map(|entry| entry.rank)
    }
}

macro_rules! rank {
    ($t:ident) => {
        impl $t {
            /// Find the minimum rank value across all entries in the dictionary.
            ///
            /// This method searches through all entries that have rank values and
            /// returns the smallest rank number. Since lower ranks typically indicate
            /// higher importance or frequency, this represents the "highest priority" entry.
            ///
            /// # Returns
            ///
            /// - `Some(u32)` - The minimum rank value if any entries have ranks
            /// - `None` - If no entries in the dictionary have rank values
            ///
            /// # Examples
            ///
            /// ```rust
            /// use odict::Dictionary;
            ///
            /// let dict = Dictionary::from_path("dictionary.xml")?;
            ///
            /// match dict.min_rank() {
            ///     Some(min_rank) => println!("Most important entry has rank: {}", min_rank),
            ///     None => println!("No entries have rank information"),
            /// }
            /// # Ok::<(), Box<dyn std::error::Error>>(())
            /// ```
            ///
            /// # Performance
            ///
            /// This operation has O(n) complexity where n is the number of entries
            /// in the dictionary, as it must examine all entries to find the minimum.
            pub fn min_rank(&self) -> Option<u32> {
                self.rank_iter().min()
            }

            /// Find the maximum rank value across all entries in the dictionary.
            ///
            /// This method searches through all entries that have rank values and
            /// returns the largest rank number. Since higher ranks typically indicate
            /// lower importance or frequency, this represents the "lowest priority" entry.
            ///
            /// # Returns
            ///
            /// - `Some(u32)` - The maximum rank value if any entries have ranks
            /// - `None` - If no entries in the dictionary have rank values
            ///
            /// # Examples
            ///
            /// ```rust
            /// use odict::Dictionary;
            ///
            /// let dict = Dictionary::from_path("dictionary.xml")?;
            ///
            /// match dict.max_rank() {
            ///     Some(max_rank) => println!("Least important entry has rank: {}", max_rank),
            ///     None => println!("No entries have rank information"),
            /// }
            /// # Ok::<(), Box<dyn std::error::Error>>(())
            /// ```
            ///
            /// # Performance
            ///
            /// This operation has O(n) complexity where n is the number of entries
            /// in the dictionary, as it must examine all entries to find the maximum.
            pub fn max_rank(&self) -> Option<u32> {
                self.rank_iter().max()
            }
        }
    };
}

rank!(Dictionary);
rank!(ArchivedDictionary);
