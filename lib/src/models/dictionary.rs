use quick_xml::de::from_str;
use rkyv::{to_bytes, Deserialize, Infallible};
use std::{collections::HashMap, error::Error};

use crate::serializable;

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
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let bytes = to_bytes::<_, 4096>(self)?;
        Ok(bytes.to_vec())
    }

    pub fn from(xml: &str) -> Result<Self, Box<dyn Error>> {
        let dict = from_str(xml)?;
        Ok(dict)
    }
}

impl From<&str> for Dictionary {
    fn from(xml: &str) -> Self {
        from_str(xml).unwrap()
    }
}

impl ArchivedDictionary {
    pub fn to_dictionary(&self) -> Result<Dictionary, Box<dyn Error>> {
        let dict: Dictionary = self.deserialize(&mut Infallible)?;
        Ok(dict)
    }
}
