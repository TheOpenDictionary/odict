use dictionary::Dictionary;
use pyo3::prelude::*;

mod dictionary;
mod types;
mod utils;

/// A Python module implemented in Rust.
#[pymodule]
fn pyodict(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Dictionary>()?;
    Ok(())
}
