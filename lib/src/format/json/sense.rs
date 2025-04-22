use serde::Serialize;
use structural_convert::StructuralConvert;

use crate::{DefinitionType, PartOfSpeech, Sense};

use super::{DefinitionJSON, EntryRefJSON, GroupJSON};

#[derive(Serialize, StructuralConvert)]
#[convert(from(DefinitionType))]
#[serde(tag = "type")]
pub enum DefinitionTypeJSON {
    #[serde(rename = "group")]
    Group(GroupJSON),

    #[serde(rename = "definition")]
    Definition(DefinitionJSON),
}

#[derive(Serialize, StructuralConvert)]
#[convert(from(Sense))]
pub struct SenseJSON {
    pub pos: PartOfSpeech,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lemma: Option<EntryRefJSON>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<DefinitionTypeJSON>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
