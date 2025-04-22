use std::fmt;

use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;

#[pyclass]
#[derive(Clone, Debug)]
pub enum FormKind {
    Conjugation,
    Inflection,
    Plural,
    Singular,
    Comparative,
    Superlative,
    PastTense,
    PresentParticiple,
    Other,
}

#[pymethods]
impl FormKind {
    fn __eq__(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl From<odict::FormKind> for FormKind {
    fn from(kind: odict::FormKind) -> Self {
        match kind {
            odict::FormKind::Conjugation => FormKind::Conjugation,
            odict::FormKind::Inflection => FormKind::Inflection,
            odict::FormKind::Plural => FormKind::Plural,
            odict::FormKind::Singular => FormKind::Singular,
            odict::FormKind::Comparative => FormKind::Comparative,
            odict::FormKind::Superlative => FormKind::Superlative,
            odict::FormKind::Other => FormKind::Other,
        }
    }
}

impl ToString for FormKind {
    fn to_string(&self) -> String {
        match self {
            FormKind::Conjugation => "conjugation".to_string(),
            FormKind::Inflection => "inflection".to_string(),
            FormKind::Plural => "plural".to_string(),
            FormKind::Singular => "singular".to_string(),
            FormKind::Comparative => "comparative".to_string(),
            FormKind::Superlative => "superlative".to_string(),
            FormKind::PastTense => "past-tense".to_string(),
            FormKind::PresentParticiple => "present-participle".to_string(),
            FormKind::Other => "other".to_string(),
        }
    }
}
