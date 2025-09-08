use either::Either;
use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::future_into_py;

use odict::ToDictionary;

use crate::{
    types::{Entry, IndexOptions, LookupOptions, LookupResult, SearchOptions, Token},
    utils::cast_error,
};

#[pyfunction]
pub fn compile(xml: String) -> PyResult<Vec<u8>> {
    let bytes = xml
        .to_dictionary()
        .and_then(|d| d.build())
        .and_then(|d| d.to_bytes())
        .map_err(cast_error)?;
    Ok(bytes)
}

#[pyclass]
pub struct OpenDictionary {
    dict: odict::OpenDictionary,
}

#[pymethods]
impl OpenDictionary {
    #[staticmethod]
    #[pyo3(signature = (dictionary, alias_path=None))]
    pub fn load<'py>(
        py: Python<'py>,
        dictionary: String,
        alias_path: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        future_into_py(py, async move {
            let mut opts = internal::LoadDictionaryOptions::default();

            if let Some(path) = alias_path {
                opts = opts.with_alias_manager(
                    odict::alias::AliasManager::new(&path).map_err(cast_error)?,
                );
            }

            let dict = internal::load_dictionary_with_options(&dictionary, opts)
                .await
                .map_err(cast_error)?;

            Ok(OpenDictionary { dict })
        })
    }

    #[new]
    pub fn new(data: Vec<u8>) -> PyResult<Self> {
        let dict = odict::OpenDictionary::from_bytes(data).map_err(cast_error)?;
        Ok(Self { dict })
    }

    #[pyo3(signature = (path, quality=None, window_size=None))]
    pub fn save(
        &mut self,
        path: String,
        quality: Option<u32>,
        window_size: Option<u32>,
    ) -> PyResult<()> {
        if quality.is_some() || window_size.is_some() {
            let mut compress_options = odict::CompressOptions::default();

            if let Some(q) = quality {
                compress_options = compress_options.quality(q);
            }

            if let Some(ws) = window_size {
                compress_options = compress_options.window_size(ws);
            }

            let compiler_options =
                odict::compile::CompilerOptions::default().with_compression(compress_options);

            self.dict
                .to_disk_with_options(&path, compiler_options)
                .map_err(cast_error)
        } else {
            self.dict.to_disk(&path).map_err(cast_error)
        }
    }

    #[getter]
    pub fn min_rank(&self) -> PyResult<Option<u32>> {
        Ok(self.dict.contents().map_err(cast_error)?.min_rank())
    }

    #[getter]
    pub fn max_rank(&self) -> PyResult<Option<u32>> {
        Ok(self.dict.contents().map_err(cast_error)?.max_rank())
    }

    #[pyo3(signature = (query, split=None, follow=None, insensitive=None))]
    pub fn lookup(
        &self,
        query: Either<String, Vec<String>>,
        split: Option<u32>,
        follow: Option<Either<bool, u32>>,
        insensitive: Option<bool>,
    ) -> PyResult<Vec<LookupResult>> {
        let mut queries: Vec<String> = vec![];

        match query {
            Either::Left(a) => queries.push(a),
            Either::Right(mut c) => queries.append(&mut c),
        }

        // Build LookupOptions from kwargs
        let options = LookupOptions {
            split,
            follow,
            insensitive,
        };

        let dict = self.dict.contents().map_err(cast_error)?;

        let results = dict
            .lookup(&queries, &odict::lookup::LookupOptions::from(options))
            .map_err(cast_error)?;

        let mapped = results
            .iter()
            .map(|result| LookupResult::from_archive(result))
            .collect::<Result<Vec<LookupResult>, _>>()?;

        Ok(mapped)
    }

    pub fn lexicon(&self) -> PyResult<Vec<&str>> {
        let dict = self.dict.contents().map_err(cast_error)?;
        let lexicon = dict.lexicon();

        Ok(lexicon)
    }

    #[pyo3(signature = (options=None))]
    pub fn index(&self, options: Option<IndexOptions>) -> PyResult<()> {
        let dict = self.dict.contents().map_err(cast_error)?;
        let opts = options.unwrap_or_default();

        dict.index(odict::index::IndexOptions::from(opts))
            .map_err(cast_error)?;

        Ok(())
    }

    #[pyo3(signature = (query, options=None))]
    pub fn search(&self, query: String, options: Option<SearchOptions>) -> PyResult<Vec<Entry>> {
        let dict = self.dict.contents().map_err(cast_error)?;
        let opts = options.unwrap_or_default();

        let results = dict
            .search(query.as_str(), odict::search::SearchOptions::from(opts))
            .map_err(cast_error)?;

        let entries = results
            .iter()
            .map(|e| e.clone().into())
            .collect::<Vec<Entry>>();

        Ok(entries)
    }

    #[pyo3(signature = (text, follow=None, insensitive=None))]
    pub fn tokenize(
        &self,
        text: String,
        follow: Option<Either<bool, u32>>,
        insensitive: Option<bool>,
    ) -> PyResult<Vec<Token>> {
        let dict = self.dict.contents().map_err(cast_error)?;

        // Build TokenizeOptions from kwargs
        let mut opts = odict::tokenize::TokenizeOptions::default();

        if let Some(f) = follow {
            opts = opts.follow(match f {
                Either::Left(true) => u32::MAX,
                Either::Left(false) => 0,
                Either::Right(num) => num,
            });
        }

        if let Some(i) = insensitive {
            opts = opts.insensitive(i);
        }

        let tokens = dict.tokenize(&text, opts).map_err(cast_error)?;

        let mut mapped_tokens = Vec::new();

        for token in tokens {
            let mut entries = Vec::new();

            for result in &token.entries {
                let entry: Entry = result.entry.deserialize().map_err(cast_error)?.into();
                println!("Tokenizing entry: {:#?}", entry);
                let directed_from = match result.directed_from {
                    Some(entry) => Some(entry.deserialize().map_err(cast_error)?.into()),
                    None => None,
                };

                entries.push(LookupResult {
                    entry,
                    directed_from,
                });
            }

            mapped_tokens.push(Token {
                lemma: token.lemma.clone(),
                language: token.language.map(|s| s.code().to_string()),
                script: token.script.name().to_string(),
                kind: format!("{:?}", token.kind),
                start: token.start,
                end: token.end,
                entries,
            });
        }

        Ok(mapped_tokens)
    }
}
