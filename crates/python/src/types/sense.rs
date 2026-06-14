use either::Either;
use internal::ToEnumWrapper;
use odict::schema::DefinitionType;
use pyo3::prelude::*;

use super::{
    definition::Definition, enums::EnumWrapper, form::Form, group::Group, translation::Translation,
};

/// A word sense — a specific meaning grouped by part of speech.
///
/// Senses represent distinct meanings of a word under a given etymology.
/// Each sense has a part of speech and contains definitions (or definition groups),
/// along with optional tags, translations, and inflected forms.
#[pyclass]
#[derive(Debug, Clone)]
pub struct Sense {
    /// The part of speech for this sense (e.g. noun, verb, adjective).
    #[pyo3(get)]
    pub pos: EnumWrapper,
    /// Optional lemma reference linking to another entry.
    #[pyo3(get)]
    pub lemma: Option<String>,
    /// Definitions or definition groups under this sense.
    #[pyo3(get)]
    pub definitions: Vec<Either<Definition, Group>>,
    /// Tags for categorizing or filtering this sense.
    #[pyo3(get)]
    pub tags: Vec<String>,
    /// Translations of this sense into other languages.
    #[pyo3(get)]
    pub translations: Vec<Translation>,
    /// Inflected forms of the word under this sense.
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
