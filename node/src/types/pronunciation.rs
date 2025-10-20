use napi_derive::napi;

use crate::types::LanguageIdentifier;

use super::media_url::MediaURL;

#[napi(object)]
pub struct Pronunciation {
    pub kind: LanguageIdentifier,
    pub value: String,
    pub media: Vec<MediaURL>,
}

impl From<odict::schema::Pronunciation> for Pronunciation {
    fn from(pronunciation: odict::schema::Pronunciation) -> Self {
        Self {
            kind: pronunciation.kind.into(),
            value: pronunciation.value,
            media: pronunciation
                .media
                .into_iter()
                .map(MediaURL::from)
                .collect(),
        }
    }
}
