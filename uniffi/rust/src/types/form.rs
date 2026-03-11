use structural_convert::StructuralConvert;
use super::enums::EnumWrapper;

#[derive(uniffi::Record, Debug, Clone, StructuralConvert)]
#[convert(from(odict::schema::Form))]
pub struct Form {
    pub kind: Option<EnumWrapper>,
    pub term: String,
    pub tags: Vec<String>,
}
