use std::error::Error;

use rkyv::{Deserialize, Infallible};

use crate::{serializable, Etymology};

serializable! {
  pub struct Entry {
    #[serde(rename = "@term")]
    pub term: String,

    #[serde(rename = "@see")]
    pub see_also: Option<String>,

    #[serde(default, rename = "ety")]
    pub etymologies: Vec<Etymology>,
  }
}

impl ArchivedEntry {
    pub fn to_entry(&self) -> Result<Entry, Box<dyn Error>> {
        let entry: Entry = self.deserialize(&mut Infallible)?;
        Ok(entry)
    }
}
