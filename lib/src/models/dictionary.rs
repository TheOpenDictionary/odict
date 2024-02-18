use quick_xml::de::from_str;
use rkyv::to_bytes;
use serde::ser::SerializeStruct;
use std::{collections::HashMap, error::Error};

use crate::{serializable, serializable_test};

use super::{entry::Entry, id::ID};

serializable_test! {
  pub struct Dictionary {
      #[serde(default, rename = "@id")]
      pub id: ID,

      #[serde(rename = "@name")]
      pub name: Option<String>,

      #[serde(default, rename = "entry", with = "entries")]
      pub entries: HashMap<String, Entry>,
  }
}

impl serde::ser::Serialize for Dictionary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut v = serializer.serialize_struct("dictionary", 3)?;

        v.serialize_field("id", &self.id)?;
        v.serialize_field("name", &self.name)?;
        v.serialize_field("entries", &self.entries)?;

        v.end()
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
