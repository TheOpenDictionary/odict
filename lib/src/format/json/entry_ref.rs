use crate::EntryRef;
use serde::{Deserialize, Serialize};

/// JSON representation of an EntryRef
/// Since EntryRef is just a string in JSON, we can simply use a wrapper around String
#[derive(Serialize, Deserialize)]
pub struct EntryRefJSON(pub String);

impl From<EntryRef> for EntryRefJSON {
    fn from(entry_ref: EntryRef) -> Self {
        Self(entry_ref.0)
    }
}

impl From<&EntryRef> for EntryRefJSON {
    fn from(entry_ref: &EntryRef) -> Self {
        Self(entry_ref.0.clone())
    }
}

impl From<EntryRefJSON> for EntryRef {
    fn from(json: EntryRefJSON) -> Self {
        Self(json.0)
    }
}

impl From<&str> for EntryRefJSON {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}
