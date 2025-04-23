use either::Either;
use odict::DefinitionType;
use pyo3::prelude::*;

use super::{definition::Definition, form::Form, group::Group, translation::Translation};

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
    #[pyo3(get)]
    pub translations: Vec<Translation>,
    #[pyo3(get)]
    pub forms: Vec<Form>,
}

impl From<odict::Sense> for Sense {
    fn from(sense: odict::Sense) -> Self {
        Sense {
            pos: sense.pos.to_string(),
            lemma: sense.lemma.map(|entry_ref| entry_ref.to_string()),
            definitions: sense
                .definitions
                .into_iter()
                .map(|def_type| match def_type {
                    DefinitionType::Definition(def) => Either::Left(def.into()),
                    DefinitionType::Group(group) => Either::Right(group.into()),
                })
                .collect(),
            tags: sense.tags,
            translations: sense.translations.into_iter().map(|t| t.into()).collect(),
            forms: sense.forms.into_iter().map(|f| f.into()).collect(),
        }
    }
}
