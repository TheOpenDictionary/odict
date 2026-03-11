use structural_convert::StructuralConvert;
use super::enums::EnumWrapper;
use super::media_url::MediaURL;

#[derive(uniffi::Record, Debug, Clone, StructuralConvert)]
#[convert(from(odict::schema::Pronunciation))]
pub struct Pronunciation {
    pub kind: Option<EnumWrapper>,
    pub value: String,
    pub media: Vec<MediaURL>,
}
