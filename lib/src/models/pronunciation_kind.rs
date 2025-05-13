use crate::serializable_enum;

serializable_enum! {
  pub enum PronunciationKind {
      #[serde(rename = "ipa")]
      IPA,
      Pinyin,
      Hiragana,
      Romaji,
      Katakana,
      Yale,     // Korean Yale romanization
      Jyutping, // Cantonese
      Bopomofo, // Zhuyin for Chinese
      Hepburn,  // Japanese romanization
      #[strum(to_string = "{0}")]
      #[serde(untagged)]
      Other(String),
  }
}
