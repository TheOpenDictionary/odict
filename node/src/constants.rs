use odict::{DictionaryReader, DictionaryWriter};
use once_cell::sync::Lazy;

pub(super) const DICT_READER: Lazy<DictionaryReader> = Lazy::new(DictionaryReader::default);
pub(super) const DICT_WRITER: Lazy<DictionaryWriter> = Lazy::new(DictionaryWriter::default);
