use serde::Serialize;

use super::DefinitionJSON;

use crate::Group;

#[derive(Serialize)]
pub struct GroupJSON {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    description: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
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
