pub mod io;

pub mod lexicon;
pub mod load;
pub mod lookup;
pub mod merge;
pub mod preview;
pub mod rank;
pub mod resolve;
pub mod semver;

pub use io::DictionaryFile;
pub(crate) use io::DictionaryReader;
pub use io::DictionaryWriter;
pub use load::DictionaryLoader;

#[cfg(feature = "tokenize-latin")]
pub use lookup::Token;

pub use rkyv::option::ArchivedOption;
