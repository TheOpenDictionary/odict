use rkyv::{
    deserialize, to_bytes,
    with::{AsBox, MapNiche},
};
use std::collections::HashSet;
use std::str::FromStr;

use crate::{error::Error, serializable};

use super::{entry::Entry, id::ID};

serializable! {
  #[derive(Default)]
  #[serde(rename = "dictionary")]
  pub struct Dictionary {
      #[serde(default, rename = "@id")]
      pub id: ID,

      #[serde(rename = "@name")]
      #[rkyv(with = MapNiche<AsBox>)]
      #[serde(skip_serializing_if = "Option::is_none")]
      pub name: Option<String>,

      #[serde(default, rename = "entry", with = "entries")]
      pub entries: HashSet<Entry>,
  }
}

mod entries {
    use std::collections::HashSet;

    use serde::de::Deserializer;
    use serde::ser::Serializer;
    use serde::Deserialize;

    use crate::models::Entry;

    pub fn serialize<S>(set: &HashSet<Entry>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(set.iter())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashSet<Entry>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut set = HashSet::new();

        for item in Vec::<Entry>::deserialize(deserializer)? {
            set.insert(item);
        }

        Ok(set)
    }
}

impl FromStr for Dictionary {
    fn from_str(xml: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(xml).map_err(|e| crate::Error::Deserialize(e.to_string()))
    }

    type Err = crate::Error;
}

impl Dictionary {
    pub fn serialize(&self) -> crate::Result<Vec<u8>> {
        let bytes =
            to_bytes::<rkyv::rancor::Error>(self).map_err(|e| Error::Serialize(e.to_string()))?;
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
