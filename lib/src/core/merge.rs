//! Dictionary merging operations for ODict.
//!
//! This module provides functionality to combine multiple dictionaries into a single
//! dictionary, preserving unique entries and avoiding duplicates.
//!
//! # Overview
//!
//! The merge operations allow you to:
//! - Merge a single dictionary into another
//! - Merge multiple dictionaries at once
//! - Preserve unique entries (no duplicates)
//!
//! # Examples
//!
//! ## Merging Two Dictionaries
//!
//! ```rust
//! use odict::Dictionary;
//!
//! let mut dict1 = Dictionary::from_path("dict1.xml")?;
//! let dict2 = Dictionary::from_path("dict2.xml")?;
//!
//! // Merge dict2 into dict1
//! dict1.merge(&dict2);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Merging Multiple Dictionaries
//!
//! ```rust
//! use odict::Dictionary;
//!
//! let mut main_dict = Dictionary::from_path("main.xml")?;
//! let dict2 = Dictionary::from_path("dict2.xml")?;
//! let dict3 = Dictionary::from_path("dict3.xml")?;
//!
//! // Merge multiple dictionaries at once
//! main_dict.merge_multi(vec![&dict2, &dict3]);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::schema::Dictionary;

impl Dictionary {
    /// Merge multiple dictionaries into this dictionary.
    ///
    /// This is a convenience method that calls [`merge`](Dictionary::merge) for each
    /// dictionary in the provided vector. Entries are processed in order, and
    /// duplicates are automatically filtered out.
    ///
    /// # Arguments
    ///
    /// * `dictionaries` - A vector of dictionary references to merge
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::Dictionary;
    ///
    /// let mut main_dict = Dictionary::from_path("main.xml")?;
    /// let dict2 = Dictionary::from_path("dict2.xml")?;
    /// let dict3 = Dictionary::from_path("dict3.xml")?;
    ///
    /// main_dict.merge_multi(vec![&dict2, &dict3]);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn merge_multi(&mut self, dictionaries: Vec<&Dictionary>) {
        for src in dictionaries {
            self.merge(src);
        }
    }

    /// Merge another dictionary into this dictionary.
    ///
    /// This method adds all entries from the source dictionary that are not
    /// already present in this dictionary. Duplicate entries (based on the
    /// entry's equality implementation) are automatically filtered out.
    ///
    /// # Arguments
    ///
    /// * `dictionary` - The source dictionary to merge from
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::Dictionary;
    ///
    /// let mut dict1 = Dictionary::from_path("dict1.xml")?;
    /// let dict2 = Dictionary::from_path("dict2.xml")?;
    ///
    /// // Merge dict2 into dict1
    /// dict1.merge(&dict2);
    ///
    /// // dict1 now contains all unique entries from both dictionaries
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Performance
    ///
    /// The merge operation has O(n) complexity where n is the number of entries
    /// in the source dictionary. Each entry is checked for existence before insertion.
    pub fn merge(&mut self, dictionary: &Dictionary) {
        for entry in dictionary.entries.iter() {
            if !self.entries.contains(entry) {
                self.entries.insert(entry.clone());
            }
        }
    }
}
