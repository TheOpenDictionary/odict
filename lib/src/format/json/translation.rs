use crate::Translation;

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct TranslationJSON {
    pub lang: String,
    pub value: String,
}

impl From<Translation> for TranslationJSON {
    fn from(translation: Translation) -> Self {
        let Translation { lang, value } = translation;

        Self { lang, value }
    }
}
