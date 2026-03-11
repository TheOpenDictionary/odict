use thiserror::Error;

#[derive(Debug, Error, uniffi::Error)]
pub enum Error {
    #[error("Odict error: {0}")]
    Odict(String),
}

impl From<odict::Error> for Error {
    fn from(e: odict::Error) -> Self {
        Error::Odict(format!("{}", e))
    }
}
