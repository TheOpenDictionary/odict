use serde::Serialize;
use structural_convert::StructuralConvert;

use crate::Definition;

use super::{ExampleJSON, NoteJSON};

#[derive(Serialize, PartialEq, Eq, StructuralConvert)]
#[convert(from(Definition))]
pub struct DefinitionJSON {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    pub value: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<ExampleJSON>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<NoteJSON>,
}

// impl From<Definition> for DefinitionJSON {
//     fn from(definition: Definition) -> Self {
//         let Definition {
//             id,
//             value,
//             examples,
//             notes,
//         } = definition;

//         Self {
//             id,
//             value,
//             examples: examples.into_iter().map(|e| ExampleJSON::from(e)).collect(),
//             notes: notes.into_iter().map(|n| NoteJSON::from(n)).collect(),
//         }
//     }
// }
