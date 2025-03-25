use std::{path::PathBuf, vec};

use wasm_bindgen::prelude::*;
use serde_wasm_bindgen;

use merge::Merge;

use crate::{
  types::{
    self, DictionaryOptions, Entry, IndexOptions, LookupOptions, LookupQuery, SearchOptions,
    SplitOptions,
  },
  utils::{cast_error, resolve_options, to_lookup_query},
};

#[wasm_bindgen]
pub struct Dictionary {
  options: Option<DictionaryOptions>,
  file: odict::DictionaryFile,
}

#[wasm_bindgen]
impl Dictionary {
  #[wasm_bindgen(constructor)]
  pub fn new(path_or_alias: String, options: Option<DictionaryOptions>) -> Result<Self, JsValue> {
    let reader = odict::DictionaryReader::default();

    let file = reader
      .read_from_path_or_alias(&path_or_alias)
      .map_err(cast_error)?;

    let dict = Dictionary { options, file };

    Ok(dict)
  }

  #[wasm_bindgen(factory)]
  pub fn write(
    xml_str: String,
    out_path: String,
    options: Option<DictionaryOptions>,
  ) -> Result<Self,JsValue> {
    let dict = odict::Dictionary::from(&xml_str).map_err(cast_error)?;
    let reader = odict::DictionaryReader::default();
    let writer = odict::DictionaryWriter::default();

    writer.write_to_path(&dict, &out_path).map_err(cast_error)?;

    let file = reader.read_from_path(&out_path).map_err(cast_error)?;

    let dict = Dictionary { options, file };

    Ok(dict)
  }

  #[wasm_bindgen(factory)]
  pub fn compile(
    xml_path: String,
    out_path: Option<String>,
    options: Option<DictionaryOptions>,
  ) -> Result<Self, JsValue> {
    let in_file = PathBuf::from(xml_path.to_owned());

    let out_file = out_path.unwrap_or_else(|| {
      odict::fs::infer_path(&xml_path)
        .to_string_lossy()
        .to_string()
    });

    let reader = odict::DictionaryReader::default();
    let writer = odict::DictionaryWriter::default();

    writer
      .compile_xml(&in_file, &out_file)
      .map_err(cast_error)?;

    let file = reader.read_from_path(&out_file).map_err(cast_error)?;

    let dict = Dictionary { options, file };

    Ok(dict)
  }

  pub fn options(&self) -> DictionaryOptions {
    resolve_options(&self.options)
  }

  #[wasm_bindgen(getter)]
  pub fn path(&self) -> Result<String,JsValue> {
    let path = self
      .file
      .path
      .as_ref()
      .map(|p| p.to_string_lossy().to_string())
      .unwrap();

    Ok(path)
  }

  pub fn _lookup(
    &self,
    queries: &Vec<odict::lookup::LookupQuery>,
    options: Option<LookupOptions>,
  ) -> Result<Vec<Vec<Entry>>, JsValue> {
    let dict = self.file.to_archive().map_err(cast_error)?;

    let mut opts = options.unwrap_or(LookupOptions::default());

    if let Some(split) = opts
      .split
      .or(self.options().split.map(|s| s.threshold).flatten())
    {
      opts.split = Some(split);
    }

    let entries = dict
      .lookup::<odict::lookup::LookupQuery, &odict::lookup::LookupOptions>(queries, &opts.into())
      .map_err(|e| cast_error(e))?;

    let mapped = entries
      .iter()
      .map(|i| {
        i.iter()
          .map(|e| Entry::from_archive(e))
          .collect::<Result<Vec<Entry>, _>>()
      })
      .collect::<Result<Vec<Vec<Entry>>, _>>()?;

    Ok(mapped)
  }

  #[wasm_bindgen]
  pub fn lookup(
    &self,
    query: JsValue,
    options: Option<LookupOptions>,
  ) -> Result<Vec<Vec<Entry>>, JsValue> {
    let mut queries: Vec<odict::lookup::LookupQuery> = vec![];

    // Handle different types of input through JsValue
    if let Some(q) = query.as_string() {
      // String case
      queries.push(q.into());
    } else if let Ok(q) = serde_wasm_bindgen::from_value::<LookupQuery>(query.clone()) {
      // LookupQuery case
      queries.push(q.into());
    } else if let Ok(q_vec) = serde_wasm_bindgen::from_value::<Vec<JsValue>>(query) {
      // Array case
      for q in q_vec {
        if let Some(s) = q.as_string() {
          queries.push(s.into());
        } else if let Ok(lq) = serde_wasm_bindgen::from_value::<LookupQuery>(q) {
          queries.push(lq.into());
        }
      }
    }

    self._lookup(&queries, options)
  }

  #[wasm_bindgen]
  pub fn lexicon(&self) -> Result<Vec<String>, JsValue> {
    let dict = self.file.to_archive().map_err(cast_error)?;
    let lexicon = dict.lexicon();

    // Convert &str to String to avoid lifetime issues
    Ok(lexicon.iter().map(|&s| s.to_string()).collect())
  }

  #[wasm_bindgen]
  pub fn split(&self, query: String, options: Option<SplitOptions>) -> Result<Vec<Entry>, JsValue> {
    let dict = self.file.to_archive().map_err(cast_error)?;

    let mut opts = options;

    opts.merge(self.options().split);

    let result = dict
      .split::<&odict::split::SplitOptions>(&query, &opts.unwrap().into())
      .map_err(|e| cast_error(e))?;

    Ok(
      result
        .iter()
        .map(|e| Entry::from_archive(e))
        .collect::<Result<Vec<Entry>, _>>()?,
    )
  }

  #[wasm_bindgen]
  pub fn index(&self, options: Option<IndexOptions>) -> Result<(), JsValue> {
    let dict = self.file.to_archive().map_err(cast_error)?;
    let mut opts = options;

    opts.merge(self.options().index);

    dict
      .index::<&odict::search::IndexOptions>(&opts.unwrap().into())
      .map_err(cast_error)?;

    Ok(())
  }

  #[wasm_bindgen]
  pub fn search(&self, query: String, options: Option<SearchOptions>) -> Result<Vec<Entry>, JsValue> {
    let dict = self.file.to_archive().map_err(cast_error)?;
    let mut opts = options;

    opts.merge(self.options().search);

    let results = dict
      .search::<&odict::search::SearchOptions>(query.as_str(), &opts.unwrap().into())
      .map_err(cast_error)?;

    let entries = results
      .iter()
      .map(|e| Entry::from_entry(e.clone()))
      .collect::<Result<Vec<Entry>, _>>()?;

    Ok(entries)
  }
}

#[cfg(test)]
mod test {
  use merge::Merge;

  #[test]
  fn test_options_merging() {
    let opts1 = crate::types::DictionaryOptions {
      search: None,
      index: Some(crate::types::IndexOptions {
        directory: Some("test".to_string()),
        memory: Some(1234),
        overwrite: Some(false),
      }),
      split: { Some(crate::types::SplitOptions { threshold: Some(5) }) },
    };

    let mut opts2: Option<crate::types::IndexOptions> = None;

    let mut opts3: Option<crate::types::SplitOptions> = Some(crate::types::SplitOptions {
      threshold: Some(10),
    });

    opts2.merge(opts1.index);
    opts3.merge(opts1.split);

    let result1 = opts2.unwrap();
    let result2 = opts3.unwrap();

    assert_eq!(result1.directory.unwrap(), "test".to_string());
    assert_eq!(result1.memory.unwrap(), 1234);
    assert_eq!(result1.overwrite.unwrap(), false);
    assert_eq!(result2.threshold.unwrap(), 10);
  }
}
