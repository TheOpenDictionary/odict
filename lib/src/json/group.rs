use serde::Serialize;

use crate::{Group, MDString};

use super::DefinitionJSON;

#[derive(Serialize)]
pub struct GroupJSON {
    id: Option<String>,
    description: Option<MDString>,
    definitions: Vec<DefinitionJSON>,
}

impl From<Group> for GroupJSON {
    fn from(group: Group) -> Self {
        let Group {
            id,
            description,
            definitions,
        } = group;

        Self {
            id,
            description,
            definitions: definitions
                .into_iter()
                .map(|d| DefinitionJSON::from(d))
                .collect(),
        }
    }
}
