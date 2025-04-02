use pyo3::prelude::*;

use super::definition::Definition;

#[pyclass]
#[derive(Debug)]
pub struct Group {
    pub id: Option<String>,
    pub description: String,
    pub definitions: Vec<Definition>,
}

impl From<odict::Group> for Group {
    fn from(group: odict::Group) -> Self {
        let odict::Group {
            id,
            description,
            definitions,
        } = group;

        Self {
            id,
            description: String::from(description),
            definitions: definitions
                .into_iter()
                .map(|d| Definition::from(d))
                .collect(),
        }
    }
}
