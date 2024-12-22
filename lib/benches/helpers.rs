use std::sync::LazyLock;

use odict::{DictionaryFile, DictionaryReader, DictionaryWriter, Result};

pub fn create_archive_dict(name: &str) -> Result<DictionaryFile> {
    let reader = DictionaryReader::default();
    let writer = DictionaryWriter::default();
    let input = format!("../examples/{}.xml", name);
    let output = format!("../examples/{}.odict", name);

    writer.compile_xml(&input, &output)?;
    reader.read_from_path(&output)
}

pub static EXAMPLE_DICTIONARY_1: LazyLock<DictionaryFile> =
    LazyLock::new(|| create_archive_dict("wiktionary").unwrap());
