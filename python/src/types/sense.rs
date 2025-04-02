use either::Either;
use pyo3::prelude::*;

use super::{definition::Definition, group::Group};

#[pyclass]
#[derive(Debug)]
pub struct Sense {
    pub pos: String,
    pub definitions: Vec<Either<Definition, Group>>,
}

impl From<odict::Sense> for Sense {
    fn from(sense: odict::Sense) -> Self {
        let odict::Sense { pos, definitions } = sense;

        Self {
            pos: pos.to_string(),
            definitions: definitions
                .into_iter()
                .map(|d| match d {
                    odict::DefinitionType::Definition(d) => Either::Left(Definition::from(d)),
                    odict::DefinitionType::Group(g) => Either::Right(Group::from(g)),
                })
                .collect(),
        }
    }
}
