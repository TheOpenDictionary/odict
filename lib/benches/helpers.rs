use std::sync::LazyLock;

use odict::{DictionaryWriter, OpenDictionary, Result};

pub async fn create_archive_dict(name: &str) -> Result<OpenDictionary> {
    let writer = DictionaryWriter::default();
    let input = format!("../examples/{}.xml", name);
    let output = format!("../examples/{}.odict", name);

    writer.compile_xml(&input, &output)?;
    OpenDictionary::from_path(&output).await.map_err(cast_error)
}

pub static EXAMPLE_DICTIONARY_1: LazyLock<OpenDictionary> =
    LazyLock::new(|| create_archive_dict("wiktionary").unwrap());
