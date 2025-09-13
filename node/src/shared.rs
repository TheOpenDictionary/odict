use std::vec;

use napi::bindgen_prelude::*;

use odict::{OpenDictionary, ToDictionary};

use crate::{
  types::{self, TokenizeOptions},
  utils::cast_error,
};

#[napi]
pub fn compile(xml: String) -> Result<Buffer> {
  let bytes = xml
    .to_dictionary()
    .and_then(|d| d.build())
    .and_then(|d| d.to_bytes())
    .map_err(cast_error)?;
  Ok(bytes.into())
}

pub fn new_from_bytes(bytes: &[u8]) -> Result<OpenDictionary> {
  let dict = odict::OpenDictionary::from_bytes::<Vec<u8>>(bytes.to_vec()).map_err(cast_error)?;
  Ok(dict)
}

pub fn get_min_rank(dict: &OpenDictionary) -> Result<Option<u32>> {
  Ok(dict.contents().map_err(cast_error)?.min_rank())
}

pub fn get_max_rank(dict: &OpenDictionary) -> Result<Option<u32>> {
  Ok(dict.contents().map_err(cast_error)?.max_rank())
}

pub fn get_lexicon(dict: &OpenDictionary) -> Result<Vec<&str>> {
  let dict = dict.contents().map_err(cast_error)?;
  let lexicon = dict.lexicon();

  Ok(lexicon)
}

pub fn perform_tokenization(
  dict: &OpenDictionary,
  text: &str,
  options: Option<TokenizeOptions>,
) -> Result<Vec<types::Token>> {
  let dict = dict.contents().map_err(cast_error)?;

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

  Ok(mapped_tokens)
}

pub fn perform_lookup(
  dict: &OpenDictionary,
  query: Either<String, Vec<String>>,
  options: Option<types::LookupOptions>,
) -> Result<Vec<types::LookupResult>> {
  let mut queries: Vec<String> = vec![];

  match query {
    Either::A(a) => queries.push(a.into()),
    Either::B(mut c) => queries.append(&mut c),
  }

  let dict = dict.contents().map_err(cast_error)?;

  let mut opts: types::LookupOptions = options.unwrap_or(types::LookupOptions::default());

  if let Some(split) = opts.split {
    opts.split = Some(split);
  }

  let results = dict
    .lookup(&queries, &odict::lookup::LookupOptions::from(opts))
    .map_err(|e| cast_error(e))?;

  let mapped = results
    .iter()
    .map(|result| result.try_into())
    .collect::<Result<Vec<types::LookupResult>>>()?;

  Ok(mapped)
}
