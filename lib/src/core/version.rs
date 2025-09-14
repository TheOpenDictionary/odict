use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SemanticVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub prerelease: Option<String>,
}

impl SemanticVersion {
    pub fn new(major: u64, minor: u64, patch: u64, prerelease: Option<String>) -> Self {
        Self {
            major,
            minor,
            patch,
            prerelease,
        }
    }

    pub fn is_compatible(&self, other: &Self) -> bool {
        self.major == other.major && self.prerelease.as_deref() == other.prerelease.as_deref()
    }

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
