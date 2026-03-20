use crate::{serializable, Error};
use url::Url;

serializable! {
  #[derive(Default)]
  #[serde(rename = "url")]
  pub struct MediaURL {
    #[serde(rename = "@src")]
    #[rkyv(with = crate::intern::Intern)]
    pub src: String,

    #[serde(rename = "@type")]
    #[rkyv(with = rkyv::with::Map<crate::intern::Intern>)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(rename = "@description")]
    #[rkyv(with = rkyv::with::Map<crate::intern::Intern>)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
  }
}

impl MediaURL {
    pub fn new(src: impl Into<String>) -> Self {
        Self {
            src: src.into(),
            mime_type: None,
            description: None,
        }
    }

    pub fn with_mime_type(mut self, mime_type: impl Into<String>) -> Self {
        self.mime_type = Some(mime_type.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn validate(&self) -> crate::Result<()> {
        // Allow relative paths and absolute URLs
        if self.src.starts_with("http://") || self.src.starts_with("https://") {
            // Validate as absolute URL
            Url::parse(&self.src).map_err(|e| Error::InvalidURL(format!("Invalid URL: {e}")))?;
        } else if !self.src.starts_with("./") && !self.src.starts_with("/") {
            // If not a relative path with ./ or / prefix, and not an absolute URL
            // then it's invalid
            return Err(Error::InvalidURL("URL must be absolute (http://, https://) or a relative path (starting with ./ or /)".to_string()));
        }
        Ok(())
    }
}
