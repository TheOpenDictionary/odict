use pyo3::prelude::*;

use super::enums::EnumWrapper;
use super::media_url::MediaURL;

use internal::ToEnumWrapper;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Pronunciation {
    #[pyo3(get)]
    pub kind: Option<EnumWrapper>,

    #[pyo3(get)]
    pub value: String,

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
