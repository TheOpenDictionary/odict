use crate::MediaURL;
use serde::Serialize;

#[derive(Serialize)]
pub struct MediaURLJSON {
    pub src: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl From<MediaURL> for MediaURLJSON {
    fn from(url: MediaURL) -> Self {
        let MediaURL {
            src,
            mime_type,
            description,
        } = url;
        Self {
            src,
            mime_type,
            description,
        }
    }
}
