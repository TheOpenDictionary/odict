use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

use crate::serializable_enum;

serializable_enum! {
  pub enum PronunciationKind {
      IPA,
      Pinyin,
      Hiragana,
      Romaji,
      Katakana,
      Yale,     // Korean Yale romanization
      Jyutping, // Cantonese
      Bopomofo, // Zhuyin for Chinese
      Hepburn,  // Japanese romanization
      Other(String),
  }
}

impl Serialize for PronunciationKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            PronunciationKind::Other(ref st) => st.to_owned(),
            _ => self.to_string(),
        };

        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for PronunciationKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(PronunciationKind::from_str(s.as_str())
            .unwrap_or(PronunciationKind::Other(s.to_string())))
    }
}
