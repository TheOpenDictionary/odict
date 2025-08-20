use std::sync::LazyLock;

use odict::{DictionaryFile, DictionaryLoader, DictionaryWriter, Result};

pub async fn create_archive_dict(name: &str) -> Result<DictionaryFile> {
    let loader = DictionaryLoader::default();
    let writer = DictionaryWriter::default();
    let input = format!("../examples/{}.xml", name);
    let output = format!("../examples/{}.odict", name);

    writer.compile_xml(&input, &output)?;
    loader.load(&output).await.map_err(cast_error)
}

pub static EXAMPLE_DICTIONARY_1: LazyLock<DictionaryFile> =
    LazyLock::new(|| create_archive_dict("wiktionary").unwrap());
