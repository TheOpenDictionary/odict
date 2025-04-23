use std::{collections::HashMap, fmt};

use pyo3::prelude::*;
use structural_convert::StructuralConvert;

use super::pronunciation::Pronunciation;
use super::sense::Sense;

#[pyclass]
#[derive(Clone, StructuralConvert)]
#[convert(from(odict::Etymology))]
pub struct Etymology {
    #[pyo3(get)]
    pub id: Option<String>,
    #[pyo3(get)]
    pub pronunciations: Vec<Pronunciation>,
    #[pyo3(get)]
    pub description: Option<String>,
    #[pyo3(get)]
    pub senses: HashMap<String, Sense>,
}

impl fmt::Debug for Etymology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut senses = self.senses.iter().collect::<Vec<(&String, &Sense)>>();

        // Sort senses alphabetically by their key
        senses.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

        f.debug_struct("Etymology")
            .field("id", &self.id)
            .field("pronunciations", &self.pronunciations)
            .field("description", &self.description)
            .field("senses", &senses)
            .finish()
    }
}
