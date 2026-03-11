use structural_convert::StructuralConvert;

#[derive(uniffi::Record, Debug, Clone, StructuralConvert)]
#[convert(from(odict::schema::Translation))]
pub struct Translation {
    pub lang: String,
    pub value: String,
}
