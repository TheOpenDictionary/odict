use std::{borrow::BorrowMut, path::PathBuf, vec};

use merge::Merge;
use napi::{bindgen_prelude::Either3, Either};

use crate::{
  types::{self, DictionaryOptions, Entry, LookupOptions, LookupQuery, MarkdownStrategy},
  utils::{cast_error, resolve_options, to_lookup_query},
};

#[napi]
pub struct Dictionary {
  pub path: String,
  options: Option<DictionaryOptions>,
  file: odict::DictionaryFile,
}

#[napi]
impl Dictionary {
  #[napi(constructor)]
  pub fn new(path: String, options: Option<DictionaryOptions>) -> napi::Result<Self> {
    let reader = odict::DictionaryReader::default();

    let file = reader.read_from_path(&path).map_err(cast_error)?;

    let dict = Dictionary {
      path,
      options,
      file,
    };

    Ok(dict)
  }

  #[napi(factory)]
  pub fn write(
    xml_str: String,
    out_path: String,
    options: Option<DictionaryOptions>,
  ) -> napi::Result<Self> {
    let dict = odict::Dictionary::from(&xml_str).map_err(cast_error)?;
    let reader = odict::DictionaryReader::default();
    let writer = odict::DictionaryWriter::default();

    writer.write_to_path(&dict, &out_path).map_err(cast_error)?;

    let file = reader.read_from_path(&out_path).map_err(cast_error)?;

    let dict = Dictionary {
      path: out_path,
      options,
      file,
    };

    Ok(dict)
  }

  #[napi(factory)]
  pub fn compile(
    xml_path: String,
    out_path: Option<String>,
    options: Option<DictionaryOptions>,
  ) -> napi::Result<Self> {
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

    let dict = Dictionary {
      path: out_file,
      options,
      file,
    };

    Ok(dict)
  }

  pub fn options(&self) -> DictionaryOptions {
    resolve_options(&self.options)
  }

  pub fn _lookup(
    &self,
    queries: &Vec<odict::lookup::LookupQuery>,
    options: Option<LookupOptions>,
  ) -> napi::Result<Vec<Vec<Entry>>> {
    let dict = self.file.to_archive().map_err(cast_error)?;

    let mut lookup_options = options.unwrap_or(LookupOptions::default());

    lookup_options.merge(LookupOptions::default());

    let LookupOptions {
      follow,
      split,
      markdown_strategy,
    } = lookup_options;

    let global_opts = self.options();
    let mut opts = odict::lookup::LookupOptions::default();

    if let Some(follow) = follow {
      opts = opts.follow(follow);
    }

    if let Some(s) = split.or(global_opts.default_split_threshold) {
      opts = opts.split(s.try_into().unwrap_or(0));
    }

    let entries = dict.lookup(queries, &opts).map_err(|e| cast_error(e))?;

    let mds = markdown_strategy
      .map(|m| MarkdownStrategy::from(m.as_str()).into())
      .unwrap_or(odict::MarkdownStrategy::Disabled);

    let mapped = entries
      .iter()
      .map(|i| {
        i.iter()
          .map(|e| Entry::from_archive(e, mds.as_ref()))
          .collect()
      })
      .collect();

    Ok(mapped)
  }

  #[napi]
  pub fn lookup(
    &self,
    query: Either3<types::LookupQuery, String, Vec<Either<LookupQuery, String>>>,
    options: Option<LookupOptions>,
  ) -> napi::Result<Vec<Vec<Entry>>> {
    let mut queries: Vec<odict::lookup::LookupQuery> = vec![];

    match query {
      Either3::A(a) => queries.push(a.into()),
      Either3::B(b) => queries.push(b.into()),
      Either3::C(c) => queries.append(
        c.into_iter()
          .map(to_lookup_query)
          .collect::<Vec<odict::lookup::LookupQuery>>()
          .borrow_mut(),
      ),
    }

    self._lookup(&queries, options)
  }

  #[napi]
  pub fn lexicon(&self) -> napi::Result<Vec<&str>> {
    let dict = self.file.to_archive().map_err(cast_error)?;
    let lexicon = dict.lexicon();

    Ok(lexicon)
  }
}
