use napi_derive::napi;
use structural_convert::StructuralConvert;

#[napi]
#[derive(StructuralConvert)]
#[convert(from(odict::PronunciationKind))]
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
