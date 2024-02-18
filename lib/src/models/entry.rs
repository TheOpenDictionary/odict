use std::error::Error;

use rkyv::{Deserialize, Infallible};
use serde::ser::SerializeStruct;

use crate::serializable_test;

use super::Etymology;

serializable_test! {
  pub struct Entry {
    #[serde(rename = "@term")]
    pub term: String,

    #[serde(rename = "@see")]
    pub see_also: Option<String>,

    #[serde(default, rename = "ety")]
    pub etymologies: Vec<Etymology>,
  }
}

impl serde::ser::Serialize for Entry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut v = serializer.serialize_struct("entry", 3)?;

        v.serialize_field("@term", &self.term)?;
        v.serialize_field("@see", &self.see_also)?;
        v.serialize_field("ety", &self.etymologies)?;

        v.end()
    }
}

impl ArchivedEntry {
    pub fn to_entry(&self) -> Result<Entry, Box<dyn Error>> {
        let entry: Entry = self.deserialize(&mut Infallible)?;
        Ok(entry)
    }
}
