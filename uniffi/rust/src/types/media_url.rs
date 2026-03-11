use structural_convert::StructuralConvert;

#[derive(uniffi::Record, Debug, Clone, StructuralConvert)]
#[convert(from(odict::schema::MediaURL))]
pub struct MediaURL {
    pub src: String,
    pub mime_type: Option<String>,
    pub description: Option<String>,
}
