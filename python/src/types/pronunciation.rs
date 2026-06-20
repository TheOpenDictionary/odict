use pyo3::prelude::*;

use super::enums::EnumWrapper;
use super::media_url::MediaURL;

use internal::ToEnumWrapper;

/// A pronunciation entry for a word or etymology.
///
/// Represents how a word is pronounced in a given notation system
/// (e.g. IPA, Pinyin), with optional audio media.
#[pyclass]
#[derive(Clone, Debug)]
pub struct Pronunciation {
    /// The pronunciation system (e.g. IPA, Pinyin), or `None`.
    #[pyo3(get)]
    pub kind: Option<EnumWrapper>,

    /// The pronunciation notation string.
    #[pyo3(get)]
    pub value: String,

    /// Audio media URLs for this pronunciation.
    #[pyo3(get)]
    pub media: Vec<MediaURL>,
}

impl From<odict::schema::Pronunciation> for Pronunciation {
    fn from(p: odict::schema::Pronunciation) -> Self {
        Self {
            kind: p.kind.map(|k| k.to_enum_wrapper().into()),
            value: p.value,
            media: p.media.into_iter().map(|m| m.into()).collect(),
        }
    }
}
