use odict::{read::DictionaryReader, DictionaryWriter};

pub struct CLIContext {
    pub reader: DictionaryReader,
    pub writer: DictionaryWriter,
}

impl CLIContext {
    pub fn default() -> Self {
        Self {
            reader: DictionaryReader::default(),
            writer: DictionaryWriter::default(),
        }
    }
}
