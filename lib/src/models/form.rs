use std::str::FromStr;

use crate::{serializable, serializable_enum};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::EntryRef;

serializable_enum! {
  pub enum FormKind {
      Conjugation,
      Inflection,
      Plural,
      Singular,
      Comparative,
      Superlative,
      Other(String),
  }
}

impl From<FormKind> for String {
    fn from(pos: FormKind) -> Self {
        match pos {
            FormKind::Other(ref s) => s.to_owned(),
            _ => pos.to_string(),
        }
    }
}

impl Serialize for FormKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            FormKind::Other(ref st) => st.to_owned(),
            _ => self.to_string(),
        };

        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for FormKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(FormKind::from_str(s.as_str()).unwrap_or(FormKind::Other(s.to_string())))
    }
}

serializable! {
  #[serde(rename = "form")]
  pub struct Form {
    #[serde(rename = "@kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<FormKind>,

    #[serde(rename = "@term")]
    pub term: EntryRef,

    #[serde(default, rename = "tag")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
  }
}
