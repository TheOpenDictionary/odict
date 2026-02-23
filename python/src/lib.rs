use dictionary::{compile, OpenDictionary};
use pyo3::prelude::*;
use types::{
    CompressOptions, EnumWrapper, IndexOptions, LoadOptions, LookupOptions, LookupResult, MediaURL,
    Pronunciation, RemoteLoadOptions, SaveOptions, SearchOptions, TokenizeOptions,
};

mod dictionary;
mod types;
mod utils;

#[pymodule]
fn theopendictionary(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Core classes
    m.add_class::<OpenDictionary>()?;

    // Types
    m.add_class::<MediaURL>()?;
    m.add_class::<Pronunciation>()?;
    m.add_class::<EnumWrapper>()?;
    m.add_class::<LookupResult>()?;

    // Options classes
    m.add_class::<LoadOptions>()?;
    m.add_class::<SaveOptions>()?;
    m.add_class::<LookupOptions>()?;
    m.add_class::<SearchOptions>()?;
    m.add_class::<IndexOptions>()?;
    m.add_class::<TokenizeOptions>()?;
    m.add_class::<CompressOptions>()?;
    m.add_class::<RemoteLoadOptions>()?;

    // Functions
    m.add_function(wrap_pyfunction!(compile, m)?)?;

    Ok(())
}
