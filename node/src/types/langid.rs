#[napi(object)]
pub struct LanguageIdentifier {
    pub language: String,
    pub script: Option<String>,
    pub region: Option<String>,
    pub variants: Vec<String>,
}

impl From<odict::schema::LanguageIdentifier> for LanguageIdentifier {
    fn from(language_identifier: odict::schema::LanguageIdentifier) -> Self {
        Self {
            language: language_identifier.language.to_string(),
            script: language_identifier.script.map(|s| s.to_string()),
            region: language_identifier.region.map(|r| r.to_string()),
            variants: language_identifier
                .variants
                .into_iter()
                .map(|v| v.to_string())
                .collect(),
        }
    }
}
