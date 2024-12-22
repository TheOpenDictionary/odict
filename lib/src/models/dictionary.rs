use quick_xml::de::from_str;
use rkyv::{deserialize, to_bytes};
use std::collections::HashMap;

use crate::{error::Error, serializable};

use super::{entry::Entry, id::ID};

serializable! {
  #[serde(rename = "dictionary")]
  pub struct Dictionary {
      #[serde(default, rename = "@id")]
      pub id: ID,

      #[serde(rename = "@name")]
      #[serde(skip_serializing_if = "Option::is_none")]
      pub name: Option<String>,

      #[serde(default, rename = "entry", with = "entries")]
      pub entries: HashMap<String, Entry>,
  }
}

mod entries {

    use std::collections::HashMap;

    use serde::de::Deserializer;
    use serde::ser::Serializer;
    use serde::Deserialize;

    use crate::models::Entry;

    pub fn serialize<S>(map: &HashMap<String, Entry>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(map.values())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, Entry>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();

        for item in Vec::<Entry>::deserialize(deserializer)? {
            map.insert(item.term.to_owned(), item);
        }

        Ok(map)
    }
}

impl Dictionary {
    pub fn serialize(&self) -> crate::Result<Vec<u8>> {
        let bytes =
            to_bytes::<rkyv::rancor::Error>(self).map_err(|e| Error::Serialize(e.to_string()))?;

        Ok(bytes.to_vec())
    }

    pub fn from(xml: &str) -> crate::Result<Self> {
        let dict = from_str(xml).map_err(|e| crate::Error::Deserialize(e.to_string()))?;
        Ok(dict)
    }
}

impl From<&str> for Dictionary {
    fn from(xml: &str) -> Self {
        from_str(xml).unwrap()
    }
}

impl ArchivedDictionary {
    pub fn to_dictionary(&self) -> crate::Result<Dictionary> {
        let dict: Dictionary = deserialize::<Dictionary, rkyv::rancor::Error>(self)
            .map_err(|e| Error::Deserialize(e.to_string()))?;

        Ok(dict)
    }
}
