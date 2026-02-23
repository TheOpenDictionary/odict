use either::Either;
use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::future_into_py;

use odict::ToDictionary;

use crate::{
    types::{Entry, IndexOptions, LoadOptions, LookupOptions, LookupResult, SearchOptions, Token},
    utils::cast_error,
};

/// Compiles an ODXML string into binary `.odict` data.
///
/// Takes an XML string conforming to the ODict XML schema and returns
/// the compiled binary representation as a byte vector. The resulting
/// bytes can be passed to [`OpenDictionary::new`] or saved to disk.
///
/// # Errors
///
/// Returns an error if the XML is malformed or does not conform to the
/// ODict schema.
#[pyfunction]
pub fn compile(xml: String) -> PyResult<Vec<u8>> {
    let bytes = xml
        .to_dictionary()
        .and_then(|d| d.build())
        .and_then(|d| d.to_bytes())
        .map_err(cast_error)?;
    Ok(bytes)
}

/// The main class for working with compiled ODict dictionaries.
///
/// An `OpenDictionary` wraps a compiled binary dictionary and provides
/// methods for looking up terms, full-text search, tokenization, and more.
///
/// # Construction
///
/// Create from compiled bytes or an XML string using [`OpenDictionary::new`],
/// or load from a file path or remote registry using [`OpenDictionary::load`].
#[pyclass]
pub struct OpenDictionary {
    dict: odict::OpenDictionary,
}

#[pymethods]
impl OpenDictionary {
    /// Loads a dictionary from a file path, alias, or remote identifier.
    ///
    /// This is an async method. If `dictionary` is a path to a `.odict` file,
    /// it loads from disk. If it matches the format `org/lang` (e.g. `wiktionary/eng`),
    /// it downloads from the remote registry.
    #[staticmethod]
    #[pyo3(signature = (dictionary, options=None))]
    pub fn load<'py>(
        py: Python<'py>,
        dictionary: String,
        options: Option<LoadOptions>,
    ) -> PyResult<Bound<'py, PyAny>> {
        future_into_py(py, async move {
            let dict = match options {
                Some(opts) => {
                    let load_opts = opts.try_into().map_err(cast_error)?;
                    odict::OpenDictionary::load_with_options(&dictionary, load_opts)
                        .await
                        .map_err(cast_error)?
                }
                None => odict::OpenDictionary::load(&dictionary)
                    .await
                    .map_err(cast_error)?,
            };

            Ok(OpenDictionary { dict })
        })
    }

    /// Creates a dictionary from compiled binary data or directly from an XML string.
    ///
    /// Accepts either `bytes` (as returned by [`compile`]) or a `str` containing
    /// ODXML markup.
    #[new]
    pub fn new(data: Either<Vec<u8>, String>) -> PyResult<Self> {
        let bytes = match data {
            Either::Left(bytes) => bytes,
            Either::Right(string) => compile(string)?,
        };
        let dict = odict::OpenDictionary::from_bytes(bytes).map_err(cast_error)?;
        Ok(Self { dict })
    }

    /// Saves the dictionary to disk as a `.odict` file.
    ///
    /// Optionally configure Brotli compression via `quality` (0–11) and
    /// `window_size` (0–22).
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

    /// The minimum rank value across all entries, or `None` if no entries have ranks.
    #[getter]
    pub fn min_rank(&self) -> PyResult<Option<u32>> {
        Ok(self.dict.contents().map_err(cast_error)?.min_rank())
    }

    /// The maximum rank value across all entries, or `None` if no entries have ranks.
    #[getter]
    pub fn max_rank(&self) -> PyResult<Option<u32>> {
        Ok(self.dict.contents().map_err(cast_error)?.max_rank())
    }

    /// Looks up one or more terms by exact match.
    ///
    /// - `query` — a single term or list of terms to look up.
    /// - `split` — minimum word length for compound splitting.
    /// - `follow` — follow `see_also` cross-references until an entry with etymologies is found.
    /// - `insensitive` — enable case-insensitive matching.
    #[pyo3(signature = (query, split=None, follow=None, insensitive=None))]
    pub fn lookup(
        &self,
        query: Either<String, Vec<String>>,
        split: Option<u32>,
        follow: Option<bool>,
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

    /// Returns all terms defined in the dictionary, sorted alphabetically.
    pub fn lexicon(&self) -> PyResult<Vec<&str>> {
        let dict = self.dict.contents().map_err(cast_error)?;
        let lexicon = dict.lexicon();

        Ok(lexicon)
    }

    /// Creates a full-text search index for the dictionary.
    ///
    /// Must be called before [`OpenDictionary::search`].
    #[pyo3(signature = (options=None))]
    pub fn index(&self, options: Option<IndexOptions>) -> PyResult<()> {
        let dict = self.dict.contents().map_err(cast_error)?;
        let opts = options.unwrap_or_default();

        dict.index(odict::index::IndexOptions::from(opts))
            .map_err(cast_error)?;

        Ok(())
    }

    /// Runs a full-text search across the dictionary.
    ///
    /// Requires an index — call [`OpenDictionary::index`] first.
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

    /// Tokenizes text using NLP-based segmentation and matches each token against the dictionary.
    ///
    /// Supports Chinese, Japanese, Korean, Thai, Khmer, German, Swedish,
    /// and Latin-script languages.
    ///
    /// - `text` — the text to tokenize.
    /// - `follow` — follow `see_also` cross-references. Accepts `True`/`False` or a number (nonzero = follow).
    /// - `insensitive` — enable case-insensitive matching.
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
                Either::Left(bool_val) => bool_val,
                Either::Right(0) => false,
                Either::Right(_) => true, // Any non-zero number means follow
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
