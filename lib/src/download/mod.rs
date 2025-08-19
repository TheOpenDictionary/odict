use crate::error::Result;

pub mod client;
pub mod download;
pub mod metadata;
pub mod utils;

pub use download::{DictionaryDownloader, DownloadOptions};
pub use utils::*;
