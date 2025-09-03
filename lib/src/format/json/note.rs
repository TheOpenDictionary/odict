use serde::Serialize;
use structural_convert::StructuralConvert;

use super::example::ExampleJSON;
use crate::schema::Note;

#[derive(Serialize, PartialEq, Eq, StructuralConvert)]
#[convert(from(Note))]
pub struct NoteJSON {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    value: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    examples: Vec<ExampleJSON>,
}
