use crate::{Form, FormKind};
use serde::Serialize;
use structural_convert::StructuralConvert;

use super::EntryRefJSON;

#[derive(Serialize, StructuralConvert)]
#[convert(from(Form))]
pub struct FormJSON {
    pub term: EntryRefJSON,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<FormKind>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
