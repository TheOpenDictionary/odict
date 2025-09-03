use crate::error::Result;

mod download;
mod metadata;
mod options;
mod remote;
mod utils;

pub use download::DictionaryDownloader;
pub use options::DownloadOptions;
pub use remote::*;
pub use utils::*;
