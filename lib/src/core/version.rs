//! Semantic versioning support for ODict dictionaries.
//!
//! This module provides a semantic versioning implementation that follows the
//! [Semantic Versioning 2.0.0](https://semver.org/) specification. It's used
//! to track dictionary format versions and ensure compatibility between different
//! versions of the ODict library.
//!
//! # Overview
//!
//! The semantic versioning functionality provides:
//! - Version parsing from strings
//! - Version comparison and ordering
//! - Compatibility checking between versions
//! - Prerelease version support
//!
//! # Compatibility Rules
//!
//! Two versions are considered compatible if:
//! - They have the same major version number
//! - They have the same prerelease status (both stable or both prerelease)
//!
//! # Examples
//!
//! ## Creating and Comparing Versions
//!
//! ```rust
//! use odict::SemanticVersion;
//!
//! let v1 = SemanticVersion::new(1, 2, 3, None);
//! let v2: SemanticVersion = "1.2.4".into();
//! let v3: SemanticVersion = "2.0.0".into();
//!
//! assert!(v1 < v2);
//! assert!(v1.is_compatible(&v2));
//! assert!(!v1.is_compatible(&v3));
//! ```
//!
//! ## Working with Prerelease Versions
//!
//! ```rust
//! use odict::SemanticVersion;
//!
//! let stable: SemanticVersion = "1.0.0".into();
//! let prerelease: SemanticVersion = "1.0.0-alpha".into();
//!
//! assert!(prerelease < stable);
//! assert!(!stable.is_compatible(&prerelease));
//! ```

use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

/// A semantic version following the Semantic Versioning 2.0.0 specification.
///
/// This struct represents a version number in the format `MAJOR.MINOR.PATCH[-PRERELEASE]`
/// where each component has specific meaning:
/// - **MAJOR**: Incremented for incompatible API changes
/// - **MINOR**: Incremented for backwards-compatible functionality additions
/// - **PATCH**: Incremented for backwards-compatible bug fixes
/// - **PRERELEASE**: Optional identifier for pre-release versions
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SemanticVersion {
    /// Major version number (incompatible API changes)
    pub major: u64,
    /// Minor version number (backwards-compatible additions)
    pub minor: u64,
    /// Patch version number (backwards-compatible fixes)
    pub patch: u64,
    /// Optional prerelease identifier (e.g., "alpha", "beta", "rc.1")
    pub prerelease: Option<String>,
}

impl SemanticVersion {
    /// Create a new semantic version.
    ///
    /// # Arguments
    ///
    /// * `major` - Major version number
    /// * `minor` - Minor version number
    /// * `patch` - Patch version number
    /// * `prerelease` - Optional prerelease identifier
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::SemanticVersion;
    ///
    /// // Stable version
    /// let stable = SemanticVersion::new(1, 2, 3, None);
    /// assert_eq!(stable.to_string(), "1.2.3");
    ///
    /// // Prerelease version
    /// let prerelease = SemanticVersion::new(1, 2, 3, Some("alpha".to_string()));
    /// assert_eq!(prerelease.to_string(), "1.2.3-alpha");
    /// ```
    pub fn new(major: u64, minor: u64, patch: u64, prerelease: Option<String>) -> Self {
        Self {
            major,
            minor,
            patch,
            prerelease,
        }
    }

    /// Check if this version is compatible with another version.
    ///
    /// Two versions are compatible if they have the same major version and
    /// the same prerelease status (both stable or both prerelease with the
    /// same identifier).
    ///
    /// # Arguments
    ///
    /// * `other` - The version to check compatibility against
    ///
    /// # Returns
    ///
    /// `true` if the versions are compatible, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::SemanticVersion;
    ///
    /// let v1_0_0: SemanticVersion = "1.0.0".into();
    /// let v1_2_3: SemanticVersion = "1.2.3".into();
    /// let v2_0_0: SemanticVersion = "2.0.0".into();
    /// let v1_0_0_alpha: SemanticVersion = "1.0.0-alpha".into();
    ///
    /// assert!(v1_0_0.is_compatible(&v1_2_3));  // Same major version
    /// assert!(!v1_0_0.is_compatible(&v2_0_0)); // Different major version
    /// assert!(!v1_0_0.is_compatible(&v1_0_0_alpha)); // Different prerelease status
    /// ```
    pub fn is_compatible(&self, other: &Self) -> bool {
        self.major == other.major && self.prerelease.as_deref() == other.prerelease.as_deref()
    }

    /// Convert the version to a byte vector.
    ///
    /// This method converts the version string representation to UTF-8 bytes,
    /// which is useful for serialization and storage in binary formats.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the UTF-8 encoded version string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::SemanticVersion;
    ///
    /// let version: SemanticVersion = "1.2.3".into();
    /// let bytes = version.as_bytes();
    /// assert_eq!(bytes, b"1.2.3");
    /// ```
    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl Display for SemanticVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut version = format!("{}.{}.{}", self.major, self.minor, self.patch);

        if let Some(prerelease) = &self.prerelease {
            version.push('-');
            version.push_str(prerelease);
        }

        write!(f, "{version}")
    }
}

impl From<&str> for SemanticVersion {
    fn from(version: &str) -> Self {
        let mut parts = version.splitn(2, '-');
        let version = parts.next().unwrap();
        let prerelease = parts.next();

        let mut parts = version.splitn(3, '.');
        let major = parts.next().unwrap().parse().unwrap();
        let minor = parts.next().unwrap().parse().unwrap();
        let patch = parts.next().unwrap().parse().unwrap();

        Self {
            major,
            minor,
            patch,
            prerelease: prerelease.map(|s| s.to_string()),
        }
    }
}

impl From<Vec<u8>> for SemanticVersion {
    fn from(version: Vec<u8>) -> Self {
        let version = std::str::from_utf8(&version).unwrap();
        Self::from(version)
    }
}

impl PartialOrd for SemanticVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let result =
            (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch));

        if result != Ordering::Equal || self.prerelease == other.prerelease {
            return Some(result);
        }

        // prerelease is considered less than the normal release
        if self.prerelease.is_some() && other.prerelease.is_none() {
            Some(Ordering::Less)
        } else if self.prerelease.is_none() && other.prerelease.is_some() {
            Some(Ordering::Greater)
        } else {
            // cannot determine order if both are prerelease but different
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
        let v1: SemanticVersion = "0.9.0".into();
        let v2: SemanticVersion = "0.11.0".into();
        assert!(v1 == v1);
        assert!(v1 < v2);
        assert!(v2 <= v2);
    }

    #[test]
    fn test_compare_prerelease() {
        let v1: SemanticVersion = "2.7.0-alpha".into();
        let v2: SemanticVersion = "2.7.0".into();
        assert!(v1 < v2);
    }
}
