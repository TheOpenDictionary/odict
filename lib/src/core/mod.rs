pub mod constants;
pub mod lexicon;
pub mod lookup;
pub mod merge;
pub mod preview;
pub mod read;
pub mod split;
pub mod write;

mod semver;

pub use read::{DictionaryFile, DictionaryReader};
pub use write::DictionaryWriter;

pub use rkyv::option::ArchivedOption;
