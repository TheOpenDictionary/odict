pub mod io;

pub mod lexicon;
pub mod lookup;
pub mod merge;
pub mod preview;

pub(crate) mod semver;

pub use io::DictionaryWriter;
pub use io::{DictionaryFile, DictionaryReader};

pub use rkyv::option::ArchivedOption;
