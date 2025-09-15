#[derive(Debug, thiserror::Error)]
pub enum Error {
    /* -------------------------------------------------------------------------- */
    /*                                     I/O                                    */
    /* -------------------------------------------------------------------------- */
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    ConversionError(#[from] std::num::TryFromIntError),

    #[error("Failed to compress: {0}")]
    Compression(String),

    #[error("Failed to write the current dictionary: {0}")]
    Write(String),

    #[error("Failed to serialize: {0}")]
    Serialize(String),

    #[error("Failed to deserialize: {0}")]
    Deserialize(String),

    #[error("Failed to print: {0}")]
    Print(String),

    #[error("This dictionary has no path!")]
    DictionaryMissingPath,

    #[cfg(feature = "alias")]
    #[error("An alias with this name already exists!")]
    AliasExists,

    #[cfg(feature = "alias")]
    #[error("Alias not found: {0}")]
    AliasNotFound(String),

    #[cfg(feature = "http")]
    #[error("Failed to download dictionary: {0} {1}")]
    DownloadFailed(crate::download::NetworkError, String),

    #[cfg(feature = "http")]
    #[error("Invalid remote dictionary name: {0}")]
    InvalidDictionaryName(String),

    #[error("This file version ({0}) is not compatible with the current version of ODict ({1})")]
    Incompatible(String, String),

    #[error("The input buffer is invalid or corrupted")]
    InvalidBuffer(#[from] rkyv::rancor::Error),

    #[error("The input does not have a valid ODict file signature")]
    InvalidSignature,

    /* -------------------------------------------------------------------------- */
    /*                                 Formatting                                 */
    /* -------------------------------------------------------------------------- */
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[cfg(feature = "sql")]
    #[error(transparent)]
    Sql(#[from] sea_query::error::Error),

    #[error("Failed to process Markdown: {0}")]
    Markdown(String),

    #[cfg(feature = "search")]
    #[error(transparent)]
    InvalidQuery(#[from] tantivy::query::QueryParserError),

    #[cfg(feature = "search")]
    #[error(transparent)]
    Index(#[from] tantivy::error::TantivyError),

    #[error("Failed to parse ID: {0}")]
    InvalidID(String),

    #[error("Invalid URL: {0}")]
    InvalidURL(String),

    #[error("An unexpected error occurred: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;
