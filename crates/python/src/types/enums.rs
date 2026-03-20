use pyo3::prelude::*;
use structural_convert::StructuralConvert;

#[pyclass]
#[derive(Debug, PartialEq, Clone, StructuralConvert)]
#[convert(from(internal::EnumWrapper))]
pub struct EnumWrapper {
    #[pyo3(get)]
    pub name: String,

    #[pyo3(get)]
    pub variant: String,

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
