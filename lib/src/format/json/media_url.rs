use crate::MediaURL;
use serde::Serialize;
use structural_convert::StructuralConvert;

#[derive(Serialize, StructuralConvert)]
#[convert(from(MediaURL))]
pub struct MediaURLJSON {
    pub src: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
