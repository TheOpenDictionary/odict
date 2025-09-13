use crate::schema::EntryRef;
use serde::{Deserialize, Serialize};
use structural_convert::StructuralConvert;

#[derive(Serialize, PartialEq, Eq, Deserialize, StructuralConvert)]
#[convert(from(EntryRef))]
pub struct EntryRefJSON(pub String);

impl From<&str> for EntryRefJSON {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}
