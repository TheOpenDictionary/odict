use odict::Error;
use pyo3::{exceptions::PyTypeError, prelude::*};

pub fn cast_error(e: Error) -> PyErr {
    PyTypeError::new_err(format!("{}", e))
}
