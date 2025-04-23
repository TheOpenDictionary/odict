use napi::{bindgen_prelude::*, Error, Result};
use napi_derive::napi;

#[napi]
pub enum PronunciationKind {
  IPA,
  Pinyin,
  Hiragana,
  Romaji,
  Katakana,
  Yale,
  Jyutping,
  Bopomofo,
  Hepburn,
  Other,
}

impl From<odict::PronunciationKind> for PronunciationKind {
  fn from(kind: odict::PronunciationKind) -> Self {
    match kind {
      odict::PronunciationKind::IPA => PronunciationKind::IPA,
      odict::PronunciationKind::Pinyin => PronunciationKind::Pinyin,
      odict::PronunciationKind::Hiragana => PronunciationKind::Hiragana,
      odict::PronunciationKind::Romaji => PronunciationKind::Romaji,
      odict::PronunciationKind::Katakana => PronunciationKind::Katakana,
      odict::PronunciationKind::Yale => PronunciationKind::Yale,
      odict::PronunciationKind::Jyutping => PronunciationKind::Jyutping,
      odict::PronunciationKind::Bopomofo => PronunciationKind::Bopomofo,
      odict::PronunciationKind::Hepburn => PronunciationKind::Hepburn,
      odict::PronunciationKind::Other => PronunciationKind::Other,
    }
  }
}
