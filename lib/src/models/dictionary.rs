use quick_xml::de::from_str;
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use super::entry::Entry;
use crate::serializable;

serializable! {
  pub struct Dictionary {
      #[serde(rename = "@id")]
     pub id: Option<String>,

      #[serde(rename = "@name")]
     pub name: Option<String>,

      #[serde(rename = "entry", with = "entries")]
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

impl From<&str> for Dictionary {
    fn from(xml: &str) -> Self {
        from_str(xml).unwrap()
    }
}

impl From<&PathBuf> for Dictionary {
    fn from(path: &PathBuf) -> Self {
        // TODO: add support for reading .odict files in addition to XML
        let contents = read_to_string(path).unwrap();
        from_str(contents.as_str()).unwrap()
    }
}
