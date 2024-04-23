use std::{borrow::BorrowMut, path::PathBuf, vec};

use napi::{bindgen_prelude::Either3, Either};
use odict::{lookup, DictionaryReader, DictionaryWriter, MarkdownStrategy};

use crate::{
  types::{self, DictionaryOptions, LookupOptions, LookupQuery},
  utils::{cast_error, resolve_options, to_lookup_query},
};

#[napi]
pub struct Dictionary {
  pub path: String,
  options: Option<DictionaryOptions>,
  file: odict::DictionaryFile,
  reader: DictionaryReader,
  writer: DictionaryWriter,
}

#[napi]
impl Dictionary {
  #[napi(constructor)]
  pub fn new(path: String, options: Option<DictionaryOptions>) -> napi::Result<Self> {
    let reader = DictionaryReader::default();
    let writer = DictionaryWriter::default();

    let file = reader.read_from_path(&path).map_err(cast_error)?;

    let dict = Dictionary {
      path,
      options,
      reader,
      writer,
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

    let reader = DictionaryReader::default();
    let writer = DictionaryWriter::default();

    writer
      .compile_xml(&in_file, &out_file)
      .map_err(cast_error)?;

    let file = reader.read_from_path(&out_file).map_err(cast_error)?;

    let dict = Dictionary {
      path: out_file,
      options,
      reader,
      writer,
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
  ) -> napi::Result<()> {
    let dict = self.file.to_archive().map_err(cast_error)?;

    let LookupOptions {
      follow,
      split,
      markdown_strategy,
    } = options.unwrap_or(LookupOptions::default());

    let global_opts = self.options();
    let mut opts = odict::lookup::LookupOptions::default();

    if let Some(follow) = follow {
      opts = opts.follow(follow);
    }

    if let Some(s) = split.or(global_opts.default_split_threshold) {
      opts = opts.split(s.try_into().unwrap_or(0));
    }

    let entries = dict.lookup(queries, &opts).map_err(|e| cast_error(e))?;

    let mapped = entries.iter().map(|i| {
      i.iter()
        .map(|e| crate::types::JSEntry::from_archive(e, &MarkdownStrategy::Disabled))
    });

    println!("{:?}", mapped);

    Ok(())
  }

  #[napi]
  pub fn lookup(
    &self,
    query: Either3<types::LookupQuery, String, Vec<Either<LookupQuery, String>>>,
    options: Option<LookupOptions>,
  ) -> napi::Result<()> {
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
}
