use structural_convert::StructuralConvert;
use super::translation::Translation;
use super::pronunciation::Pronunciation;

#[derive(uniffi::Record, Debug, Clone, StructuralConvert)]
#[convert(from(odict::schema::Example))]
pub struct Example {
    pub value: String,
    pub translations: Vec<Translation>,
    pub pronunciations: Vec<Pronunciation>,
}
