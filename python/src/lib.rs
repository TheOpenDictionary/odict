use dictionary::Dictionary;
use pyo3::prelude::*;

mod dictionary;
mod types;
mod utils;

#[pymodule]
fn theopendictionary(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Dictionary>()?;
    Ok(())
}
