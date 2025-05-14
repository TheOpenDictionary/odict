use dictionary::Dictionary;
use pyo3::prelude::*;
use types::{EnumWrapper, MediaURL, Pronunciation};

mod dictionary;
mod types;
mod utils;

#[pymodule]
fn theopendictionary(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Dictionary>()?;
    m.add_class::<MediaURL>()?;
    m.add_class::<Pronunciation>()?;
    m.add_class::<EnumWrapper>()?;
    Ok(())
}
