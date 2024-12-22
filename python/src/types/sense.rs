use either::Either;
use pyo3::prelude::*;

use super::{definition::Definition, group::Group};

#[pyclass]
#[derive(Debug)]
pub struct Sense {
    pub pos: String,
    pub definitions: Vec<Either<Definition, Group>>,
}

impl Sense {
    pub fn from(sense: odict::Sense) -> PyResult<Self> {
        let odict::Sense { pos, definitions } = sense;

        Ok(Self {
            pos: pos.to_string(),
            definitions: definitions
                .into_iter()
                .map(|d| -> PyResult<Either<Definition, Group>> {
                    match d {
                        odict::DefinitionType::Definition(d) => {
                            Ok(Either::Left(Definition::from(d)?))
                        }
                        odict::DefinitionType::Group(g) => Ok(Either::Right(Group::from(g)?)),
                    }
                })
                .collect::<Result<Vec<Either<Definition, Group>>, _>>()?,
        })
    }
}
