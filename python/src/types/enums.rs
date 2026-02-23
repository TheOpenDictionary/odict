use pyo3::prelude::*;
use structural_convert::StructuralConvert;

/// A wrapper for ODict enumeration values (e.g. part of speech, pronunciation kind).
///
/// ODict enums are represented as string triples: the enum name,
/// the variant name, and the variant's string value.
#[pyclass]
#[derive(Debug, PartialEq, Clone, StructuralConvert)]
#[convert(from(internal::EnumWrapper))]
pub struct EnumWrapper {
    /// The enum type name (e.g. `"PartOfSpeech"`).
    #[pyo3(get)]
    pub name: String,

    /// The variant name (e.g. `"Noun"`).
    #[pyo3(get)]
    pub variant: String,

    /// The string value of the variant (e.g. `"n"`).
    #[pyo3(get)]
    pub value: String,
}

#[pymethods]
impl EnumWrapper {
    fn __str__(&self) -> String {
        self.value.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "EnumWrapper(variant='{}', value='{}')",
            self.variant, self.value
        )
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.variant == other.variant && self.value == other.value
    }
}
