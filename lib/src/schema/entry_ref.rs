use std::ops::Deref;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{serializable, serializable_custom};

serializable_custom! {
  #[derive(Default)]
  pub struct EntryRef(
      pub Box<super::Entry>
  );
}

impl Serialize for EntryRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.term.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EntryRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let term = String::deserialize(deserializer)?;
        Ok(EntryRef(Box::new(super::Entry {
            term,
            see_also: None,
            etymologies: Vec::new(),
            media: Vec::new(),
            rank: None,
        })))
    }
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
