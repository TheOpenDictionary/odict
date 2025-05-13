use serde::Serialize;
use structural_convert::StructuralConvert;

use super::TranslationJSON;
use crate::Example;

#[derive(Serialize, PartialEq, Eq, StructuralConvert)]
#[convert(from(Example))]
pub struct ExampleJSON {
    pub value: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translations: Vec<TranslationJSON>,
}
