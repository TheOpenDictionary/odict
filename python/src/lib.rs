use dictionary::Dictionary;
use pyo3::prelude::*;
use types::Form;

mod dictionary;
mod types;
mod utils;

#[pymodule]
fn theopendictionary(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Dictionary>()?;
    m.add_class::<Form>()?;
    Ok(())
}
