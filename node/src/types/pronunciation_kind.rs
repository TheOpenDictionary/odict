use napi_derive::napi;
use structural_convert::StructuralConvert;

use super::media_url::MediaURL;

#[napi]
#[derive(StructuralConvert)]
#[convert(from(odict::PronunciationKind))]
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
  #[convert(from(skip))]
  Other(String),
}
