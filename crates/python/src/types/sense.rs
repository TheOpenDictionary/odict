use either::Either;
use internal::ToEnumWrapper;
use odict::schema::DefinitionType;
use pyo3::prelude::*;

use super::{
    definition::Definition, enums::EnumWrapper, form::Form, group::Group, translation::Translation,
};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Sense {
    #[pyo3(get)]
    pub pos: EnumWrapper,
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

impl From<odict::schema::Sense> for Sense {
    fn from(sense: odict::schema::Sense) -> Self {
        Sense {
            pos: sense.pos.to_enum_wrapper().into(),
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
