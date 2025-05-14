use crate::Translation;
use structural_convert::StructuralConvert;

use serde::Serialize;

#[derive(Serialize, Clone, StructuralConvert)]
#[convert(from(Translation))]
pub struct TranslationJSON {
    pub lang: String,
    pub value: String,
}
