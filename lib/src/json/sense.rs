use serde::Serialize;

use crate::{DefinitionType, PartOfSpeech, Sense};

use super::{DefinitionJSON, GroupJSON};

#[derive(Serialize)]
pub enum DefinitionTypeJSON {
    Group(GroupJSON),
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
    pub definitions: Vec<DefinitionTypeJSON>,
}

impl From<Sense> for SenseJSON {
    fn from(sense: Sense) -> Self {
        let Sense { pos, definitions } = sense;

        Self {
            pos,
            definitions: definitions
                .into_iter()
                .map(|d| DefinitionTypeJSON::from(d))
                .collect(),
        }
    }
}
