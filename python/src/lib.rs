use dictionary::Dictionary;
use pyo3::prelude::*;
use types::{FormKind, MediaURL, Pronunciation, PronunciationKind};

mod dictionary;
mod types;
mod utils;

#[pymodule]
fn theopendictionary(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Dictionary>()?;
    m.add_class::<FormKind>()?;
    m.add_class::<PronunciationKind>()?;
    m.add_class::<MediaURL>()?;
    m.add_class::<Pronunciation>()?;
    Ok(())
}
