use std::collections::HashMap;

use pyo3::prelude::*;

use super::{mdstring::MDString, sense::Sense};

#[pyclass]
pub struct Etymology {
    pub id: Option<String>,
    pub pronunciation: Option<String>,
    pub description: Option<MDString>,
    pub senses: HashMap<String, Sense>,
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
            description: description.map(|d| MDString::from(d)),
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
