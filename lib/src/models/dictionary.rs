use quick_xml::de::from_str;
use rkyv::to_bytes;
use std::{collections::HashMap, error::Error};

use super::{entry::Entry, id::ID};
use crate::serializable;

serializable! {
  pub struct Dictionary {
      #[serde(default, rename = "@id", )]
     pub id: ID,

      #[serde(rename = "@name")]
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
}

impl From<&str> for Dictionary {
    fn from(xml: &str) -> Self {
        from_str(xml).unwrap()
    }
}

pub trait ToDictionary {
    fn to_dictionary(&self) -> Result<Dictionary, Box<dyn Error>>;
}

impl ToDictionary for String {
    fn to_dictionary(&self) -> Result<Dictionary, Box<dyn Error>> {
        Ok(Dictionary::from(self.as_str()))
    }
}
