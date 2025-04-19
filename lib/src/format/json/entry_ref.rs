use crate::EntryRef;
use serde::{Deserialize, Serialize};

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
