use std::{str::FromStr, sync::LazyLock};

use odict::{schema::Dictionary, OpenDictionary, Result};
use tempfile::NamedTempFile;

pub fn get_example_dict(name: &str) -> Result<OpenDictionary> {
    let input = format!("../examples/{}.xml", name);
    let output = NamedTempFile::new()?.path().to_str().unwrap().to_string();

    Dictionary::from_str(&input)?.to_disk(&output)?;

    tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime")
        .block_on(async { OpenDictionary::from_path(&output) })
}

pub static EXAMPLE_DICT_1: LazyLock<OpenDictionary> =
    LazyLock::new(|| get_example_dict("example1").expect("Failed to get example dictionary 1"));

pub static EXAMPLE_DICT_2: LazyLock<OpenDictionary> =
    LazyLock::new(|| get_example_dict("example2").expect("Failed to get example dictionary 2"));
