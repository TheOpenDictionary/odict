use std::error::Error;

use rkyv::{archived_root, to_bytes, Deserialize, Infallible};

use crate::{serializable, Etymology};

serializable! {
  pub struct Entry {
    #[serde(rename = "@term")]
    pub term: String,

    #[serde(rename = "@see")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<String>,

    #[serde(default, rename = "ety")]
    pub etymologies: Vec<Etymology>,
  }
}

impl Entry {
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let bytes = to_bytes::<_, 4096>(self)?;
        Ok(bytes.to_vec())
    }
}

impl ArchivedEntry {
    pub fn to_entry(&self) -> Result<Entry, Box<dyn Error>> {
        let entry: Entry = self.deserialize(&mut Infallible)?;
        Ok(entry)
    }
}
