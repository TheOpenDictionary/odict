#[derive(Debug, thiserror::Error)]
pub enum Error {
    /* -------------------------------------------------------------------------- */
    /*                                     I/O                                    */
    /* -------------------------------------------------------------------------- */
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[error("Failed to compress: {0}")]
    Compression(#[from] lz4_flex::frame::Error),

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

    #[error("An alias with this name already exists!")]
    AliasExists,

    #[error("This file is not compatible with the current version of ODict")]
    Incompatible,

    #[error("The input does not have a valid ODict file signature")]
    InvalidSignature,

    /* -------------------------------------------------------------------------- */
    /*                                 Formatting                                 */
    /* -------------------------------------------------------------------------- */
    #[error(transparent)]
    Json(#[from] serde_json::Error),

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

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
