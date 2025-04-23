use serde::Serialize;

use super::media_url::MediaURLJSON;
use crate::{Pronunciation, PronunciationKind};

#[derive(Serialize)]
pub struct PronunciationJSON {
    pub kind: PronunciationKind,

    pub value: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub urls: Vec<MediaURLJSON>,
}

impl From<Pronunciation> for PronunciationJSON {
    fn from(pronunciation: Pronunciation) -> Self {
        Self {
            kind: pronunciation.kind,
            value: pronunciation.value,
            urls: pronunciation
                .urls
                .into_iter()
                .map(MediaURLJSON::from)
                .collect(),
        }
    }
}
