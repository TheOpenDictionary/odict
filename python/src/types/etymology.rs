use std::{collections::HashMap, fmt};

use pyo3::prelude::*;

use super::sense::Sense;

#[pyclass]
pub struct Etymology {
    pub id: Option<String>,
    pub pronunciation: Option<String>,
    pub description: Option<String>,
    pub senses: HashMap<String, Sense>,
}

impl fmt::Debug for Etymology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut senses = self.senses.iter().collect::<Vec<(&String, &Sense)>>();

        // Sort senses alphabetically by their key
        senses.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

        f.debug_struct("Etymology")
            .field("id", &self.id)
            .field("pronunciation", &self.pronunciation)
            .field("description", &self.description)
            .field("senses", &senses)
            .finish()
    }
}

impl Etymology {
    pub fn from(etymology: odict::Etymology) -> PyResult<Self> {
        let odict::Etymology {
            id,
            pronunciation,
            description,
            senses,
        } = etymology;

        Ok(Self {
            id,
            pronunciation,
            description: description.map(|d| String::from(d)),
            senses: senses
                .into_iter()
                .map(|(k, v)| -> PyResult<(String, Sense)> {
                    let sense = Sense::from(v)?;
                    Ok((k.to_string(), sense))
                })
                .collect::<PyResult<HashMap<String, Sense>>>()?,
        })
    }
}
