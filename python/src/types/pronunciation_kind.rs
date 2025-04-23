use std::fmt;

use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use structural_convert::StructuralConvert;

#[pyclass]
#[derive(Clone, Debug, PartialEq, StructuralConvert)]
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

#[pymethods]
impl PronunciationKind {
    #[new]
    pub fn new(kind: &str) -> Self {
        match kind.to_lowercase().as_str() {
            "ipa" => Self::IPA,
            "pinyin" => Self::Pinyin,
            "hiragana" => Self::Hiragana,
            "romaji" => Self::Romaji,
            "katakana" => Self::Katakana,
            "yale" => Self::Yale,
            "jyutping" => Self::Jyutping,
            "bopomofo" => Self::Bopomofo,
            "hepburn" => Self::Hepburn,
            _ => Self::Other,
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        match self {
            Self::IPA => "PronunciationKind.IPA".to_string(),
            Self::Pinyin => "PronunciationKind.Pinyin".to_string(),
            Self::Hiragana => "PronunciationKind.Hiragana".to_string(),
            Self::Romaji => "PronunciationKind.Romaji".to_string(),
            Self::Katakana => "PronunciationKind.Katakana".to_string(),
            Self::Yale => "PronunciationKind.Yale".to_string(),
            Self::Jyutping => "PronunciationKind.Jyutping".to_string(),
            Self::Bopomofo => "PronunciationKind.Bopomofo".to_string(),
            Self::Hepburn => "PronunciationKind.Hepburn".to_string(),
            Self::Other => "PronunciationKind.Other".to_string(),
        }
    }
}

impl fmt::Display for PronunciationKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IPA => write!(f, "ipa"),
            Self::Pinyin => write!(f, "pinyin"),
            Self::Hiragana => write!(f, "hiragana"),
            Self::Romaji => write!(f, "romaji"),
            Self::Katakana => write!(f, "katakana"),
            Self::Yale => write!(f, "yale"),
            Self::Jyutping => write!(f, "jyutping"),
            Self::Bopomofo => write!(f, "bopomofo"),
            Self::Hepburn => write!(f, "hepburn"),
            Self::Other => write!(f, "other"),
        }
    }
}
