use structural_convert::StructuralConvert;

use super::etymology::Etymology;
use super::media_url::MediaURL;

#[napi(object)]
#[derive(StructuralConvert)]
#[convert(from(odict::schema::Entry))]
pub struct Entry {
    pub term: String,
    pub rank: Option<u32>,
    pub see_also: Option<String>,
    pub etymologies: Vec<Etymology>,
    pub media: Vec<MediaURL>,
}
