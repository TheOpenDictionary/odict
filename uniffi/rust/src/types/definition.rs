use structural_convert::StructuralConvert;
use super::example::Example;

#[derive(uniffi::Record, Debug, Clone, StructuralConvert)]
#[convert(from(odict::schema::Definition))]
pub struct Definition {
    pub id: Option<String>,
    pub value: String,
    pub examples: Vec<Example>,
}
