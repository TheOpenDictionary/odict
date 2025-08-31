use std::vec;

use napi::bindgen_prelude::*;

use odict::ToDictionary;

#[cfg(feature = "node")]
use crate::types::LoadOptions;
#[cfg(feature = "node")]
use crate::types::SaveOptions;
use crate::{
  types::{self, Entry},
  utils::cast_error,
};

#[napi]
pub struct OpenDictionary {
  dict: odict::OpenDictionary,
}

#[napi]
pub fn compile(xml: String) -> Result<Buffer> {
  let bytes = xml
    .to_dictionary()
    .and_then(|d| d.build())
    .and_then(|d| d.to_bytes())
    .map_err(cast_error)?;
  Ok(bytes.into())
}

#[napi]
impl OpenDictionary {
  #[cfg(feature = "node")]
  #[napi(factory)]
  pub async fn load(dictionary: String, options: Option<LoadOptions>) -> Result<Self> {
    use internal::LoadDictionaryOptions;

    let opts = options
      .map(|o| LoadDictionaryOptions::try_from(o))
      .transpose()
      .map_err(cast_error)?
      .unwrap_or_default();

    let dict = internal::load_dictionary_with_options(&dictionary, opts)
      .await
      .map_err(cast_error)?;

    Ok(Self { dict })
  }

  #[napi(constructor)]
  pub fn new(data: Buffer) -> Result<Self> {
    let dict = odict::OpenDictionary::from_bytes::<Vec<u8>>(data.into()).map_err(cast_error)?;
    Ok(Self { dict })
  }

  #[cfg(feature = "node")]
  #[napi]
  pub fn save(&mut self, path: String, options: Option<SaveOptions>) -> Result<()> {
    match options {
      Some(opts) => {
        let compiler_options = odict::compile::CompilerOptions::from(opts);
        self
          .dict
          .to_disk_with_options(&path, compiler_options)
          .map_err(cast_error)
      }
      None => self.dict.to_disk(&path).map_err(cast_error),
    }
  }

  pub fn _lookup(
    &self,
    queries: &Vec<String>,
    options: Option<types::LookupOptions>,
  ) -> Result<Vec<types::LookupResult>> {
    let dict = self.dict.contents().map_err(cast_error)?;

    let mut opts: types::LookupOptions = options.unwrap_or(types::LookupOptions::default());

    if let Some(split) = opts.split {
      opts.split = Some(split);
    }

    let results = dict
      .lookup(queries, &odict::lookup::LookupOptions::from(opts))
      .map_err(|e| cast_error(e))?;

    let mapped = results
      .iter()
      .map(|result| result.try_into())
      .collect::<Result<Vec<types::LookupResult>>>()?;

    Ok(mapped)
  }

  #[napi(getter)]
  pub fn min_rank(&self) -> Result<Option<u32>> {
    Ok(self.dict.contents().map_err(|e| cast_error(e))?.min_rank())
  }

  #[napi(getter)]
  pub fn max_rank(&self) -> Result<Option<u32>> {
    Ok(self.dict.contents().map_err(|e| cast_error(e))?.max_rank())
  }

  #[napi]
  pub fn lookup(
    &self,
    query: Either<String, Vec<String>>,
    options: Option<types::LookupOptions>,
  ) -> Result<Vec<types::LookupResult>> {
    let mut queries: Vec<String> = vec![];

    match query {
      Either::A(a) => queries.push(a.into()),
      Either::B(mut c) => queries.append(&mut c),
    }

    self._lookup(&queries, options)
  }

  #[napi]
  pub fn lexicon(&self) -> Result<Vec<&str>> {
    let dict = self.dict.contents().map_err(cast_error)?;
    let lexicon = dict.lexicon();

    Ok(lexicon)
  }

  #[napi]
  pub fn index(&self, options: Option<types::IndexOptions>) -> Result<()> {
    #[cfg(feature = "node")]
    {
      let dict = self.dict.contents().map_err(cast_error)?;
      let opts = options.unwrap_or(types::IndexOptions::default());

      dict
        .index(odict::index::IndexOptions::from(opts))
        .map_err(cast_error)?;

      return Ok(());
    }

    #[cfg(not(feature = "node"))]
    unimplemented!("index() is not available in browser environments. Maybe try IndexedDB?");
  }

  #[napi]
  pub fn search(&self, query: String, options: Option<types::SearchOptions>) -> Result<Vec<Entry>> {
    #[cfg(feature = "node")]
    {
      let dict = self.dict.contents().map_err(cast_error)?;
      let opts = options.unwrap_or_default();

      // Use our helper function to avoid orphan rule issues
      let results = dict
        .search(query.as_str(), odict::search::SearchOptions::from(opts))
        .map_err(cast_error)?;

      // Use the new from_entry function for Entry types
      let entries = results
        .iter()
        .map(|e| e.clone().into())
        .collect::<Vec<Entry>>();

      return Ok(entries);
    }

    #[cfg(not(feature = "node"))]
    unimplemented!("search() is not available in browser environments. Maybe try IndexedDB?");
  }

  #[napi]
  pub fn tokenize(
    &self,
    text: String,
    options: Option<types::TokenizeOptions>,
  ) -> Result<Vec<types::Token>> {
    #[cfg(feature = "tokenize")]
    {
      let dict = self.dict.contents().map_err(cast_error)?;

      let opts = match options {
        Some(o) => o.into(),
        None => odict::tokenize::TokenizeOptions::default(),
      };

      let tokens = dict.tokenize(&text, opts).map_err(cast_error)?;

      // Convert tokens manually
      let mut mapped_tokens = Vec::new();

      for token in tokens {
        let mut entries = Vec::new();

        for result in &token.entries {
          let entry: types::Entry = result.entry.deserialize().map_err(cast_error)?.into();

          let directed_from = match result.directed_from {
            Some(entry) => Some(entry.deserialize().map_err(cast_error)?.into()),
            None => None,
          };

          entries.push(types::LookupResult {
            entry,
            directed_from,
          });
        }

        mapped_tokens.push(types::Token {
          lemma: token.lemma.clone(),
          language: token.language.map(|s| s.code().to_string()),
          script: token.script.name().to_string(),
          kind: format!("{:?}", token.kind),
          start: token.start as u16,
          end: token.end as u16,
          entries,
        });
      }

      return Ok(mapped_tokens);
    }

    #[cfg(not(feature = "tokenize"))]
    unimplemented!("tokenize() is not available in this environment.");
  }
}

#[cfg(test)]
mod test {
  use merge::Merge;

  #[test]
  fn test_options_merging() {
    let mut opts1 = crate::types::DictionaryOptions {
      search: None,
      index: Some(crate::types::IndexOptions {
        directory: Some("test".to_string()),
        memory: Some(1234),
        overwrite: Some(false),
      }),
      split: {
        Some(crate::types::SplitOptions {
          min_length: Some(5),
        })
      },
    };

    let opts2 = crate::types::DictionaryOptions {
      search: Some(crate::types::SearchOptions {
        directory: Some("search_dir".to_string()),
        threshold: Some(10),
        autoindex: Some(true),
        limit: Some(100),
      }),
      index: None,
      split: None,
    };

    opts1.merge(opts2);

    assert_eq!(
      opts1.search.as_ref().unwrap().directory,
      Some("search_dir".to_string())
    );

    assert_eq!(
      opts1.index.as_ref().unwrap().directory,
      Some("test".to_string())
    );
    assert_eq!(opts1.split.as_ref().unwrap().min_length, Some(5));
    assert_eq!(opts1.index.as_ref().unwrap().memory, Some(1234));
    assert_eq!(opts1.index.as_ref().unwrap().overwrite, Some(false));
    assert_eq!(opts1.search.as_ref().unwrap().threshold, Some(10));
    assert_eq!(opts1.search.as_ref().unwrap().autoindex, Some(true));
    assert_eq!(opts1.search.as_ref().unwrap().limit, Some(100));
  }
}
