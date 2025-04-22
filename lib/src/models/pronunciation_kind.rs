use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

use crate::serializable_custom;

serializable_custom! {
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
      Other,
  }
}

impl Serialize for PronunciationKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for PronunciationKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let lowercase = s.to_lowercase();

        match lowercase.as_str() {
            "ipa" => Ok(PronunciationKind::IPA),
            "pinyin" => Ok(PronunciationKind::Pinyin),
            "hiragana" => Ok(PronunciationKind::Hiragana),
            "romaji" => Ok(PronunciationKind::Romaji),
            "katakana" => Ok(PronunciationKind::Katakana),
            "yale" => Ok(PronunciationKind::Yale),
            "jyutping" => Ok(PronunciationKind::Jyutping),
            "bopomofo" => Ok(PronunciationKind::Bopomofo),
            "hepburn" => Ok(PronunciationKind::Hepburn),
            _ => Ok(PronunciationKind::Other),
        }
    }
}

impl fmt::Display for PronunciationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PronunciationKind::IPA => write!(f, "ipa"),
            PronunciationKind::Pinyin => write!(f, "pinyin"),
            PronunciationKind::Hiragana => write!(f, "hiragana"),
            PronunciationKind::Romaji => write!(f, "romaji"),
            PronunciationKind::Katakana => write!(f, "katakana"),
            PronunciationKind::Yale => write!(f, "yale"),
            PronunciationKind::Jyutping => write!(f, "jyutping"),
            PronunciationKind::Bopomofo => write!(f, "bopomofo"),
            PronunciationKind::Hepburn => write!(f, "hepburn"),
            PronunciationKind::Other => write!(f, "other"),
        }
    }
}
