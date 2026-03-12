use serde::Serialize;
use structural_convert::StructuralConvert;

use super::media_url::MediaURLJSON;
use crate::schema::{Pronunciation, PronunciationKind};

#[derive(Serialize, PartialEq, Eq, StructuralConvert)]
#[convert(from(Pronunciation))]
pub struct PronunciationJSON {
    pub kind: Option<PronunciationKind>,

    pub value: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub media: Vec<MediaURLJSON>,
}
