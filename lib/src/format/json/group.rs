use serde::Serialize;
use structural_convert::StructuralConvert;

use super::DefinitionJSON;

use crate::schema::Group;

#[derive(Serialize, PartialEq, Eq, StructuralConvert)]
#[convert(from(Group))]
pub struct GroupJSON {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    description: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    definitions: Vec<DefinitionJSON>,
}
