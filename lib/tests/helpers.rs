use std::sync::LazyLock;

use odict::{schema::Dictionary, OpenDictionary, Result};
use tempfile::NamedTempFile;

pub fn get_example_dict(name: &str) -> Result<OpenDictionary> {
    let input = format!("../examples/{}.xml", name);
    let output = NamedTempFile::new()?.path().to_str().unwrap().to_string();
    let mut dict = Dictionary::from_path(&input).unwrap().build().unwrap();

    dict.to_disk(&output).unwrap();

    Ok(dict)
}

#[allow(dead_code)]
pub static EXAMPLE_DICT_1: LazyLock<OpenDictionary> =
    LazyLock::new(|| get_example_dict("example1").expect("Failed to get example dictionary 1"));

#[allow(dead_code)]
pub static EXAMPLE_DICT_2: LazyLock<OpenDictionary> =
    LazyLock::new(|| get_example_dict("example2").expect("Failed to get example dictionary 2"));
