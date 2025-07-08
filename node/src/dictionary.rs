use std::vec;

use napi::bindgen_prelude::*;

use merge::Merge;
use odict::ToDictionary;

use crate::{
  types::{self, DictionaryOptions, Entry},
  utils::{cast_error, resolve_options},
};

#[napi]
pub struct Dictionary {
  options: Option<DictionaryOptions>,
  file: odict::DictionaryFile,
}

#[napi]
pub fn compile(xml: String) -> Result<Buffer> {
  let dictionary = xml.to_dictionary().map_err(cast_error)?;

  let bytes = odict::DictionaryWriter::default()
    .write_to_bytes(&dictionary)
    .map_err(cast_error)?;

  Ok(bytes.into())
}

#[napi]
impl Dictionary {
  #[napi(constructor)]
  pub fn new(data: Buffer, options: Option<DictionaryOptions>) -> Result<Self> {
    let reader = odict::DictionaryReader::default();
    let file = reader
      .read_from_bytes::<Vec<u8>>(data.into())
      .map_err(cast_error)?;
    let dict = Dictionary { options, file };

    Ok(dict)
  }

  pub fn options(&self) -> DictionaryOptions {
    resolve_options(&self.options)
  }

  pub fn _lookup(
    &self,
    queries: &Vec<String>,
    options: Option<types::LookupOptions>,
  ) -> Result<Vec<types::LookupResult>> {
    let dict = self.file.to_archive().map_err(cast_error)?;

    let mut opts: types::LookupOptions = options.unwrap_or(types::LookupOptions::default());

    if let Some(split) = opts
      .split
      .or(self.options().split.map(|s| s.min_length).flatten())
    {
      opts.split = Some(split);
    }

    // Fix: Convert to odict::lookup::LookupOptions explicitly to help type inference
    let odict_options: odict::lookup::LookupOptions = opts.into();
    let results = dict
      .lookup(queries, odict_options)
      .map_err(|e| cast_error(e))?;

    // Convert results properly
    let mapped = results
      .iter()
      .map(|result| result.try_into())
      .collect::<Result<Vec<types::LookupResult>>>()?;

    Ok(mapped)
  }

  #[napi(getter)]
  pub fn min_rank(&self) -> Result<Option<u32>> {
    Ok(
      self
        .file
        .to_archive()
        .map_err(|e| cast_error(e))?
        .min_rank(),
    )
  }

  #[napi(getter)]
  pub fn max_rank(&self) -> Result<Option<u32>> {
    Ok(
      self
        .file
        .to_archive()
        .map_err(|e| cast_error(e))?
        .max_rank(),
    )
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
    let dict = self.file.to_archive().map_err(cast_error)?;
    let lexicon = dict.lexicon();

    Ok(lexicon)
  }

  #[napi]
  pub fn index(&self, options: Option<types::IndexOptions>) -> Result<()> {
    #[cfg(feature = "search")]
    {
      let dict = self.file.to_archive().map_err(cast_error)?;
      let mut opts = options.unwrap_or(types::IndexOptions::default());

      if let Some(default_opts) = self.options().index {
        opts.merge(default_opts);
      }

      dict
        .index(odict::search::IndexOptions::from(opts))
        .map_err(cast_error)?;

      return Ok(());
    }

    #[cfg(not(feature = "search"))]
    unimplemented!("index() is not available in browser environments. Maybe try IndexedDB?");
  }

  #[napi]
  pub fn search(&self, query: String, options: Option<types::SearchOptions>) -> Result<Vec<Entry>> {
    #[cfg(feature = "search")]
    {
      let dict = self.file.to_archive().map_err(cast_error)?;
      let mut opts = options.unwrap_or_default();

      if let Some(default_opts) = self.options().search {
        opts.merge(default_opts);
      }

      // Use our helper function to avoid orphan rule issues
      let results = dict
        .search(query.as_str(), odict::search::SearchOptions::from(opts))
        .map_err(cast_error)?;

      // Use the new from_entry function for Entry types
      let entries = results
        .iter()
        .map(|e| types::entry::from_entry(e))
        .collect::<Result<Vec<Entry>>>()?;

      return Ok(entries);
    }

    #[cfg(not(feature = "search"))]
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
      let dict = self.file.to_archive().map_err(cast_error)?;

      let opts = match options {
        Some(o) => o.into(),
        None => odict::lookup::TokenizeOptions::default(),
      };

      let tokens = dict.tokenize(&text, opts).map_err(cast_error)?;

      // Convert tokens manually
      let mut mapped_tokens = Vec::new();

      for token in tokens {
        let mut entries = Vec::new();

        for result in &token.entries {
          let entry = types::entry::from_archive(result.entry)?;

          let directed_from = match result.directed_from {
            Some(entry) => Some(types::entry::from_archive(entry)?),
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
    let opts1 = crate::types::DictionaryOptions {
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

    let mut opts2: Option<crate::types::IndexOptions> = None;

    let mut opts3: Option<crate::types::SplitOptions> = Some(crate::types::SplitOptions {
      min_length: Some(10),
    });

    opts2.merge(opts1.index);
    opts3.merge(opts1.split);

    let result1 = opts2.unwrap();
    let result2 = opts3.unwrap();

    assert_eq!(result1.directory.unwrap(), "test".to_string());
    assert_eq!(result1.memory.unwrap(), 1234);
    assert_eq!(result1.overwrite.unwrap(), false);
    assert_eq!(result2.min_length.unwrap(), 10);
  }
}
