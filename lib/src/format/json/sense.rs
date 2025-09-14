use std::hash::Hash;

use serde::Serialize;
use structural_convert::StructuralConvert;

use crate::schema::{DefinitionType, PartOfSpeech, Sense};

use super::{DefinitionJSON, EntryRefJSON, FormJSON, GroupJSON, TranslationJSON};

#[derive(Serialize, PartialEq, Eq, StructuralConvert)]
#[convert(from(DefinitionType))]
#[serde(tag = "type")]
pub enum DefinitionTypeJSON {
    #[serde(rename = "group")]
    Group(GroupJSON),

    #[serde(rename = "definition")]
    Definition(DefinitionJSON),
}

#[derive(Serialize, PartialEq, Eq, StructuralConvert)]
#[convert(from(Sense))]
pub struct SenseJSON {
    pub pos: PartOfSpeech,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lemma: Option<EntryRefJSON>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<DefinitionTypeJSON>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translations: Vec<TranslationJSON>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub forms: Vec<FormJSON>,
}

impl Hash for SenseJSON {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}
