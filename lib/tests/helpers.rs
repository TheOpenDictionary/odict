use std::{error::Error, sync::LazyLock};

use odict::{DictionaryFile, DictionaryReader, DictionaryWriter};
use tempfile::NamedTempFile;

pub fn get_example_dict(name: &str) -> crate::Result<DictionaryFile> {
    let reader = DictionaryReader::default();
    let writer = DictionaryWriter::default();
    let input = format!("../examples/{}.xml", name);
    let output = NamedTempFile::new()?.path().to_str().unwrap().to_string();

    writer.compile_xml(&input, &output)?;

    reader.read_from_path(&output)
}

pub static EXAMPLE_DICT_1: LazyLock<DictionaryFile> =
    LazyLock::new(|| get_example_dict("example1").expect("Failed to get example dictionary 1"));

pub static EXAMPLE_DICT_2: LazyLock<DictionaryFile> =
    LazyLock::new(|| get_example_dict("example2").expect("Failed to get example dictionary 2"));
