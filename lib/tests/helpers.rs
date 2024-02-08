use once_cell::sync::Lazy;
use std::error::Error;

use odict::{DictionaryFile, DictionaryReader, DictionaryWriter};

pub fn create_archive_dict(name: &str) -> Result<DictionaryFile, Box<dyn Error>> {
    let reader = DictionaryReader::default();
    let writer = DictionaryWriter::default();
    let input = format!("../examples/{}.xml", name);
    let output = format!("../examples/{}.odict", name);

    writer.compile_xml(&input, &output)?;
    reader.read_from_path(&output)
}

pub static EXAMPLE_DICTIONARY_1: Lazy<DictionaryFile> =
    Lazy::new(|| create_archive_dict("example1").unwrap());
