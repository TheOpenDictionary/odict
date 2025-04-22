use serde::Serialize;

use super::TranslationJSON;
use crate::Example;

#[derive(Serialize, Clone)]
pub struct ExampleJSON {
    pub value: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translations: Vec<TranslationJSON>,
}

impl From<Example> for ExampleJSON {
    fn from(example: Example) -> Self {
        let Example {
            value,
            translations,
            ..
        } = example;

        Self {
            value,
            translations: translations
                .into_iter()
                .map(|t| TranslationJSON::from(t))
                .collect(),
        }
    }
}

impl From<&Example> for ExampleJSON {
    fn from(example: &Example) -> Self {
        Self {
            value: example.value.clone(),
            translations: example
                .translations
                .iter()
                .map(|t| TranslationJSON::from(t.clone()))
                .collect(),
        }
    }
}
