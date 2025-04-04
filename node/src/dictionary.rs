use std::vec;

use napi::bindgen_prelude::*;

use merge::Merge;
use odict::ToDictionary;

use crate::{
  types::{self, DictionaryOptions, Entry, IndexOptions, LookupOptions, SearchOptions},
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
    options: Option<LookupOptions>,
  ) -> Result<Vec<types::LookupResult>> {
    let dict = self.file.to_archive().map_err(cast_error)?;

    let mut opts: LookupOptions = options.unwrap_or(LookupOptions::default());

    if let Some(split) = opts
      .split
      .or(self.options().split.map(|s| s.min_length).flatten())
    {
      opts.split = Some(split);
    }

    let results = dict
      .lookup(queries, &odict::lookup::LookupOptions::from(opts))
      .map_err(|e| cast_error(e))?;

    let mapped = results
      .iter()
      .map(|result| {
        let entry = Entry::from_archive(result.entry)?;
        let directed_from = match &result.directed_from {
          Some(from) => Some(Entry::from_archive(from)?),
          None => None,
        };

        Ok(types::LookupResult {
          entry,
          directed_from,
        })
      })
      .collect::<Result<Vec<types::LookupResult>, _>>()?;

    Ok(mapped)
  }

  #[napi]
  pub fn lookup(
    &self,
    query: Either<String, Vec<String>>,
    options: Option<LookupOptions>,
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
  pub fn index(&self, options: Option<IndexOptions>) -> Result<()> {
    #[cfg(feature = "search")]
    {
      let dict = self.file.to_archive().map_err(cast_error)?;
      let mut opts = options;

      opts.merge(self.options().index);

      dict
        .index::<&odict::search::IndexOptions>(&opts.unwrap().into())
        .map_err(cast_error)?;

      return Ok(());
    }

    unimplemented!("index() is not available in browser environments. Maybe try IndexedDB?");
  }

  #[napi]
  pub fn search(&self, query: String, options: Option<SearchOptions>) -> Result<Vec<Entry>> {
    #[cfg(feature = "search")]
    {
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

      return Ok(entries);
    }

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

      let mapped = tokens
        .iter()
        .map(|token| {
          let entries = token
            .entries
            .iter()
            .map(|result| {
              let entry = Entry::from_archive(result.entry)?;
              let directed_from = match &result.directed_from {
                Some(from) => Some(Entry::from_archive(from)?),
                None => None,
              };

              Ok(types::LookupResult {
                entry,
                directed_from,
              })
            })
            .collect::<Result<Vec<types::LookupResult>, _>>()?;

          Ok(types::Token {
            lemma: token.lemma.clone(),
            language: token.language.map(|s| s.code().to_string()).clone(),
            script: token.script.name().to_string(),
            kind: format!("{:?}", token.kind),
            start: token.start as u16,
            end: token.end as u16,
            entries,
          })
        })
        .collect::<Result<Vec<types::Token>, _>>()?;

      return Ok(mapped);
    }

    unimplemented!("tokenize() is not available in this environment.");

    Ok(vec![])
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
