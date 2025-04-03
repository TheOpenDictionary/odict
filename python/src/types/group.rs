use pyo3::prelude::*;

use super::definition::Definition;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Group {
    #[pyo3(get)]
    pub id: Option<String>,
    #[pyo3(get)]
    pub description: String,
    #[pyo3(get)]
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
