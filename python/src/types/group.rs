use pyo3::prelude::*;

use super::definition::Definition;

#[pyclass]
#[derive(Debug)]
pub struct Group {
    pub id: Option<String>,
    pub description: String,
    pub definitions: Vec<Definition>,
}

impl Group {
    pub fn from(group: odict::Group) -> PyResult<Self> {
        let odict::Group {
            id,
            description,
            definitions,
        } = group;

        Ok(Self {
            id,
            description: String::from(description),
            definitions: definitions
                .into_iter()
                .map(|d| Definition::from(d))
                .collect::<Result<Vec<Definition>, _>>()?,
        })
    }
}
