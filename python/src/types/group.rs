use pyo3::prelude::*;

use super::{definition::Definition, MDString};

#[pyclass]
#[derive(Debug)]
pub struct Group {
    pub id: Option<String>,
    pub description: MDString,
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
            description: MDString::from(description),
            definitions: definitions
                .into_iter()
                .map(|d| Definition::from(d))
                .collect::<Result<Vec<Definition>, _>>()?,
        })
    }
}
