use std::collections::HashMap;

use crate::serializable;

use super::{pos::PartOfSpeech, sense::Sense, MDString};

serializable! {
  pub struct Etymology {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "@pronunciation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation: Option<String>,

    #[serde(rename = "@description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<MDString>,

    #[serde(rename = "sense", with = "senses")]
    pub senses: HashMap<PartOfSpeech, Sense>,
  }
}

mod senses {

    use std::collections::HashMap;

    use serde::de::Deserializer;
    use serde::ser::Serializer;
    use serde::Deserialize;

    use crate::models::{PartOfSpeech, Sense};

    pub fn serialize<S>(
        map: &HashMap<PartOfSpeech, Sense>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(map.values())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<PartOfSpeech, Sense>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();

        for item in Vec::<Sense>::deserialize(deserializer)? {
            map.insert(item.pos.clone(), item);
        }

        Ok(map)
    }
}
