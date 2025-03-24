use crate::types::*;
use crate::utils::cast_error;
use js_sys::Array;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Dictionary {
    file: odict::DictionaryFile,
}

#[wasm_bindgen]
impl Dictionary {
    #[wasm_bindgen(constructor)]
    pub fn new(path_or_alias: &str) -> Result<Dictionary, JsValue> {
        let reader = odict::DictionaryReader::default();

        let file = reader
            .read_from_path_or_alias(path_or_alias)
            .map_err(cast_error)?;

        Ok(Dictionary { file })
    }

    #[wasm_bindgen]
    pub fn write(xml_str: &str, out_path: &str) -> Result<Dictionary, JsValue> {
        let dict = odict::Dictionary::from(xml_str).map_err(cast_error)?;
        let reader = odict::DictionaryReader::default();
        let writer = odict::DictionaryWriter::default();

        writer.write_to_path(&dict, out_path).map_err(cast_error)?;

        let file = reader.read_from_path(out_path).map_err(cast_error)?;

        Ok(Dictionary { file })
    }

    #[wasm_bindgen]
    pub fn search(&self, query: &str, options: Option<SearchOptions>) -> Result<Array, JsValue> {
        let dict = self.file.to_archive().map_err(cast_error)?;
        
        let search_options = match options {
            Some(opts) => opts.into(),
            None => odict::search::SearchOptions::default(),
        };

        let results = dict
            .search::<odict::search::SearchOptions>(query, &search_options)
            .map_err(cast_error)?;

        let entries = Array::new();
        
        for result in results {
            let entry = Entry::from(&result.entry.to_entry().map_err(cast_error)?);
            entries.push(&JsValue::from_serde(&entry).unwrap());
        }

        Ok(entries)
    }

    #[wasm_bindgen]
    pub fn to_json(&self) -> Result<String, JsValue> {
        let dict = self.file.to_dictionary().map_err(cast_error)?;
        let json = serde_json::to_string(&dict).map_err(cast_error)?;
        Ok(json)
    }

    #[wasm_bindgen]
    pub fn to_md(&self) -> Result<String, JsValue> {
        let dict = self.file.to_dictionary().map_err(cast_error)?;
        let md = odict::format::md::to_md(&dict).map_err(cast_error)?;
        Ok(md)
    }
}
