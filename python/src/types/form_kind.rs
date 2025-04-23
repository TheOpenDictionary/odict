use pyo3::prelude::*;
use structural_convert::StructuralConvert;

#[pyclass]
#[derive(Clone, Debug, StructuralConvert)]
#[convert(from(odict::FormKind))]
pub enum FormKind {
    Conjugation,
    Inflection,
    Plural,
    Singular,
    Comparative,
    Superlative,
    Other,
}

#[pymethods]
impl FormKind {
    fn __eq__(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
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
            FormKind::Other => "other".to_string(),
        }
    }
}
