use std::{path::PathBuf, str::FromStr};

use either::Either;
use odict::{
    lookup::{LookupOptions, LookupStrategy},
    search::{IndexOptions, SearchOptions},
};
use pyo3::prelude::*;

use crate::{
    types::{Entry, LookupResult, Token},
    utils::cast_error,
};

fn lookup(
    file: &odict::DictionaryFile,
    queries: &Vec<String>,
    split: Option<usize>,
    follow: Option<Either<bool, u32>>,
    insensitive: Option<bool>,
) -> PyResult<Vec<crate::types::LookupResult>> {
    let dict = file.to_archive().map_err(cast_error)?;

    let mut opts = LookupOptions::default();

    if let Some(split) = split {
        opts.strategy = LookupStrategy::Split(split);
    }

    if let Some(follow) = follow {
        let follow_value = match follow {
            Either::Left(true) => u32::MAX,
            Either::Left(false) => return Ok(vec![]), // Don't set follow, equivalent to None
            Either::Right(num) => num,
        };
        opts.follow = Some(follow_value);
    }

    if let Some(insensitive) = insensitive {
        opts.insensitive = insensitive;
    }

    let results = dict
        .lookup(queries, &odict::lookup::LookupOptions::from(opts.into()))
        .map_err(|e| cast_error(e))?;

    let mapped = results
        .iter()
        .map(|result| crate::types::LookupResult::from_archive(result))
        .collect::<Result<Vec<crate::types::LookupResult>, _>>()?;

    Ok(mapped)
}

#[pyclass]
pub struct Dictionary {
    file: odict::DictionaryFile,
}

#[pymethods]
impl Dictionary {
    #[new]
    pub fn new(path_or_alias: String) -> PyResult<Self> {
        let reader = odict::DictionaryReader::default();

        let file = reader
            .read_from_path_or_alias(&path_or_alias)
            .map_err(cast_error)?;

        let dict = Dictionary { file };

        Ok(dict)
    }

    #[staticmethod]
    pub fn write(xml_str: String, out_path: String) -> PyResult<Self> {
        let dict = odict::Dictionary::from_str(&xml_str).map_err(cast_error)?;
        let reader = odict::DictionaryReader::default();
        let writer = odict::DictionaryWriter::default();

        writer.write_to_path(&dict, &out_path).map_err(cast_error)?;

        let file = reader.read_from_path(&out_path).map_err(cast_error)?;

        let dict = Dictionary { file };

        Ok(dict)
    }

    #[staticmethod]
    #[pyo3(signature = (xml_path, out_path=None))]
    pub fn compile(xml_path: String, out_path: Option<String>) -> PyResult<Self> {
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

        let dict = Dictionary { file };

        Ok(dict)
    }

    #[getter]
    pub fn path(&self) -> PyResult<String> {
        let path = self
            .file
            .path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap();

        Ok(path)
    }

    #[getter]
    pub fn min_rank(&self) -> PyResult<Option<u32>> {
        Ok(self.file.to_archive().map_err(cast_error)?.min_rank())
    }

    #[getter]
    pub fn max_rank(&self) -> PyResult<Option<u32>> {
        Ok(self.file.to_archive().map_err(cast_error)?.max_rank())
    }

    #[pyo3(signature = (query, split=None, follow=None, insensitive=None))]
    pub fn lookup(
        &self,
        query: Either<String, Vec<String>>,
        split: Option<usize>,
        follow: Option<Either<bool, u32>>,
        insensitive: Option<bool>,
    ) -> PyResult<Vec<LookupResult>> {
        let mut queries: Vec<String> = vec![];

        match query {
            Either::Left(a) => queries.push(a.into()),
            Either::Right(mut c) => queries.append(&mut c),
        }

        lookup(&(self.file), &queries, split, follow, insensitive)
    }

    pub fn lexicon(&self) -> PyResult<Vec<&str>> {
        let dict = self.file.to_archive().map_err(cast_error)?;
        let lexicon = dict.lexicon();

        Ok(lexicon)
    }

    #[pyo3(signature = (directory=None, memory=None, overwrite=None))]
    pub fn index(
        &self,
        directory: Option<String>,
        memory: Option<usize>,
        overwrite: Option<bool>,
    ) -> PyResult<()> {
        let dict = self.file.to_archive().map_err(cast_error)?;
        let mut opts = IndexOptions::default();

        if let Some(directory) = directory {
            opts = opts.dir(&directory);
        }

        if let Some(memory) = memory {
            opts = opts.memory(memory);
        }

        if let Some(overwrite) = overwrite {
            opts = opts.overwrite(overwrite);
        }

        dict.index(&opts).map_err(cast_error)?;

        Ok(())
    }

    #[pyo3(signature = (query, directory=None, threshold=None, autoindex=None, limit=None))]
    pub fn search(
        &self,
        query: String,
        directory: Option<String>,
        threshold: Option<u32>,
        autoindex: Option<bool>,
        limit: Option<usize>,
    ) -> PyResult<Vec<Entry>> {
        let dict = self.file.to_archive().map_err(cast_error)?;
        let mut opts = SearchOptions::default();

        if let Some(directory) = directory {
            opts = opts.dir(&directory);
        }

        if let Some(threshold) = threshold {
            opts = opts.threshold(threshold);
        }

        if let Some(autoindex) = autoindex {
            opts = opts.autoindex(autoindex);
        }

        if let Some(limit) = limit {
            opts = opts.limit(limit);
        }

        let results = dict
            .search::<&odict::search::SearchOptions>(query.as_str(), &opts)
            .map_err(cast_error)?;

        let entries = results
            .iter()
            .map(|e| Entry::from(e.clone()))
            .collect::<Vec<Entry>>();

        Ok(entries)
    }

    #[pyo3(signature = (text, follow=None, insensitive=None))]
    pub fn tokenize(
        &self,
        text: String,
        follow: Option<Either<bool, u32>>,
        insensitive: Option<bool>,
    ) -> PyResult<Vec<crate::types::Token>> {
        let dict = self.file.to_archive().map_err(cast_error)?;

        let mut opts = odict::lookup::TokenizeOptions::default();

        if let Some(follow) = follow {
            match follow {
                Either::Left(true) => {
                    opts = opts.follow(u32::MAX);
                }
                Either::Left(false) => {
                    // Don't set follow, use default behavior
                }
                Either::Right(num) => {
                    opts = opts.follow(num);
                }
            };
        }

        if let Some(insensitive) = insensitive {
            opts = opts.insensitive(insensitive);
        }

        let tokens = dict.tokenize(&text, opts).map_err(cast_error)?;

        let mapped = tokens
            .iter()
            .map(|token| Ok(Token::from(token.clone())))
            .collect::<Result<Vec<crate::types::Token>, PyErr>>()?;

        Ok(mapped)
    }
}
