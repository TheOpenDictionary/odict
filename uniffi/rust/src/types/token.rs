use super::lookup::LookupResult;

#[derive(uniffi::Record)]
pub struct Token {
    pub lemma: String,
    pub language: Option<String>,
    pub script: String,
    pub kind: String,
    pub start: u32,
    pub end: u32,
    pub entries: Vec<LookupResult>,
}
