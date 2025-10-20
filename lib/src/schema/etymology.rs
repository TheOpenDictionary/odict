use std::collections::HashSet;

use crate::schema::pronunciation::Pronunciation;
use crate::serializable;

use super::sense::Sense;

serializable! {
  #[derive(Default)]
  #[serde(rename = "ety")]
  pub struct Etymology {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[rkyv(with = rkyv::with::Map<rkyv_intern::Intern>)]
    pub id: Option<String>,

    #[serde(default, rename = "pronunciation")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pronunciations: Vec<Pronunciation>,

    #[serde(rename = "sense", default, with = "senses")]
    pub senses: HashSet<Sense>,

    #[serde(rename = "@description")]
    #[rkyv(with = rkyv::with::Map<rkyv_intern::Intern>)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
  }
}

mod senses {
    use std::collections::HashSet;

    use serde::de::Deserializer;
    use serde::ser::Serializer;
    use serde::Deserialize;

    use crate::schema::Sense;

    pub fn serialize<S>(set: &HashSet<Sense>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(set.iter())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashSet<Sense>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let senses = Vec::<Sense>::deserialize(deserializer)?;
        Ok(senses.into_iter().collect())
    }
}
