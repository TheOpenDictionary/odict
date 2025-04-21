use either::Either;
use pyo3::prelude::*;

use super::{definition::Definition, group::Group};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Sense {
    #[pyo3(get)]
    pub pos: String,
    #[pyo3(get)]
    pub lemma: Option<String>,
    #[pyo3(get)]
    pub definitions: Vec<Either<Definition, Group>>,
    #[pyo3(get)]
    pub tags: Vec<String>,
}

impl From<odict::Sense> for Sense {
    fn from(sense: odict::Sense) -> Self {
        let odict::Sense {
            pos,
            lemma,
            definitions,
            tags,
        } = sense;

        Self {
            pos: pos.to_string(),
            lemma: lemma.map(|l| l.0),
            definitions: definitions
                .into_iter()
                .map(|d| match d {
                    odict::DefinitionType::Definition(d) => Either::Left(Definition::from(d)),
                    odict::DefinitionType::Group(g) => Either::Right(Group::from(g)),
                })
                .collect(),
            tags,
        }
    }
}
