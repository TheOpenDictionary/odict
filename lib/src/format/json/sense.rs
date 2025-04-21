use serde::Serialize;

use crate::{DefinitionType, PartOfSpeech, Sense};

use super::{DefinitionJSON, EntryRefJSON, GroupJSON};

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum DefinitionTypeJSON {
    #[serde(rename = "group")]
    Group(GroupJSON),

    #[serde(rename = "definition")]
    Definition(DefinitionJSON),
}

impl From<DefinitionType> for DefinitionTypeJSON {
    fn from(definition: DefinitionType) -> Self {
        match definition {
            DefinitionType::Group(g) => DefinitionTypeJSON::Group(GroupJSON::from(g)),
            DefinitionType::Definition(d) => {
                DefinitionTypeJSON::Definition(DefinitionJSON::from(d))
            }
        }
    }
}

#[derive(Serialize)]
pub struct SenseJSON {
    pub pos: PartOfSpeech,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lemma: Option<EntryRefJSON>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<DefinitionTypeJSON>,
    
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl From<Sense> for SenseJSON {
    fn from(sense: Sense) -> Self {
        let Sense {
            pos,
            lemma,
            definitions,
            tags,
        } = sense;

        Self {
            pos,
            lemma: lemma.map(EntryRefJSON::from),
            definitions: definitions
                .into_iter()
                .map(|d| DefinitionTypeJSON::from(d))
                .collect(),
            tags,
        }
    }
}
