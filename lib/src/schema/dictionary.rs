use indexmap::IndexSet;
use rkyv::deserialize;

use std::str::FromStr;

use crate::{error::Error, intern::serialize_interned, serializable};

use super::{entry::Entry, id::ID};

pub type EntryList = IndexSet<Entry>;

serializable! {
  #[derive(Default)]
  #[serde(rename = "dictionary")]
  pub struct Dictionary {
      #[serde(default, rename = "@id")]
      pub id: ID,

      #[serde(rename = "@name")]
      #[rkyv(with = rkyv::with::Map<crate::intern::Intern>)]
      #[serde(skip_serializing_if = "Option::is_none")]
      pub name: Option<String>,

      #[serde(default, rename = "entry")]
      pub entries: EntryList
  }
}

impl FromStr for Dictionary {
    fn from_str(xml: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(xml).map_err(|e| crate::Error::Deserialize(e.to_string()))
    }

    type Err = crate::Error;
}

impl Dictionary {
    pub(crate) fn serialize(&self) -> crate::Result<Vec<u8>> {
        let bytes = serialize_interned::<_, rkyv::rancor::Error>(self)?;
        Ok(bytes.to_vec())
    }
}

impl ArchivedDictionary {
    pub fn deserialize(&self) -> crate::Result<Dictionary> {
        let dict: Dictionary = deserialize::<Dictionary, rkyv::rancor::Error>(self)
            .map_err(|e| Error::Deserialize(e.to_string()))?;
        Ok(dict)
    }
}
