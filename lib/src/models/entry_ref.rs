use std::ops::Deref;

use crate::serializable;

serializable! {
  pub struct EntryRef(pub String);
}

impl EntryRef {
    pub fn new(s: impl Into<String>) -> Self {
        EntryRef(s.into())
    }
}

impl Deref for EntryRef {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ArchivedEntryRef {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for EntryRef {
    fn from(s: String) -> Self {
        EntryRef(s)
    }
}

impl From<EntryRef> for String {
    fn from(entry_ref: EntryRef) -> Self {
        entry_ref.0
    }
}

impl From<&str> for EntryRef {
    fn from(s: &str) -> Self {
        EntryRef(s.to_string())
    }
}

impl AsRef<str> for EntryRef {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for EntryRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
