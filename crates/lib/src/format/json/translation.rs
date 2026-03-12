use crate::schema::Translation;
use structural_convert::StructuralConvert;

use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, StructuralConvert)]
#[convert(from(Translation))]
pub struct TranslationJSON {
    pub lang: String,
    pub value: String,
}
