#[derive(Debug, thiserror::Error)]
pub enum Error {
    /* -------------------------------------------------------------------------- */
    /*                                     I/O                                    */
    /* -------------------------------------------------------------------------- */
    #[error("Failed to read the current dictionary: {0}")]
    Read(#[from] std::io::Error),

    #[error("Failed to write the current dictionary: {0}")]
    Write(String),

    #[error("Failed to parse the current dictionary: {0}")]
    Parse(#[from] quick_xml::DeError),

    #[error("Failed to serialize the current dictionary: {0}")]
    Serialize(String),

    #[error("Failed to deserialize the current dictionary: {0}")]
    Deserialize(String),

    #[error("Failed to print: `{0}`")]
    Print(String),

    #[error("This file is not compatible with the current version of ODict")]
    Incompatible,

    #[error("The input does not have a valid ODict file signature")]
    InvalidSignature,

    /* -------------------------------------------------------------------------- */
    /*                                 Formatting                                 */
    /* -------------------------------------------------------------------------- */
    #[error("Failed to format the current dictionary as JSON: `{0}`")]
    JSON(#[from] serde_json::Error),

    #[error("Failed to format the current dictionary as SQL: `{0}`")]
    SQL(#[from] sea_query::Error),

    /* -------------------------------------------------------------------------- */
    /*                               Search & Index                               */
    /* -------------------------------------------------------------------------- */
    #[error("Failed to search the current dictionary: `{0}`")]
    InvalidQuery(#[from] tantivy::query::QueryParserError),

    #[error("Failed to index the current dictionary: `{0}`")]
    Index(#[from] tantivy::error::TantivyError),

    /* -------------------------------------------------------------------------- */
    /*                                    Other                                   */
    /* -------------------------------------------------------------------------- */
    #[error("An unknown exception has occurred: `{0}`")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;
