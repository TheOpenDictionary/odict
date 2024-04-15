use std::{error::Error, ffi::OsStr, path::PathBuf};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rkyv::{archived_root, Deserialize, Infallible};
use tantivy::{
    collector::TopDocs, query::QueryParser, tokenizer::TextAnalyzer, Index, ReloadPolicy,
};

use crate::{Dictionary, Entry};

use super::constants::{CUSTOM_TOKENIZER, DEFAULT_TOKENIZER};

use super::{
    get_default_index_dir,
    schema::{FIELD_BUFFER, FIELD_DEFINITIONS, FIELD_TERM},
};

pub struct SearchOptions {
    pub dir: PathBuf,
    pub threshold: u32,
    pub limit: usize,
    pub tokenizer: TextAnalyzer,
}

impl SearchOptions {
    pub fn default() -> Self {
        Self {
            dir: get_default_index_dir(),
            threshold: 1,
            limit: 10,
            tokenizer: DEFAULT_TOKENIZER.to_owned(),
        }
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn tokenizer<T>(mut self, tokenizer: T) -> Self
    where
        TextAnalyzer: From<T>,
    {
        self.tokenizer = tokenizer.into();
        self
    }

    pub fn threshold(mut self, threshold: u32) -> Self {
        self.threshold = threshold;
        self
    }

    pub fn dir<P: AsRef<OsStr> + ?Sized>(mut self, dir: &P) -> Self {
        self.dir = PathBuf::from(dir);
        self
    }
}

impl AsRef<SearchOptions> for SearchOptions {
    fn as_ref(&self) -> &SearchOptions {
        self
    }
}

impl Dictionary {
    pub fn search<Options: AsRef<SearchOptions>>(
        &self,
        query: &str,
        options: Options,
    ) -> Result<Vec<Entry>, Box<dyn Error>> {
        let opts = options.as_ref();
        let index_path = opts.dir.join(self.id.as_str());
        let index = Index::open_in_dir(&index_path)?;

        index
            .tokenizers()
            .register(CUSTOM_TOKENIZER, opts.tokenizer.to_owned());

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()?;

        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(&index, vec![*FIELD_TERM, *FIELD_DEFINITIONS]);
        let query_obj = query_parser.parse_query(query)?;
        let top_docs = searcher.search(&query_obj, &TopDocs::with_limit(opts.limit))?;
        let entries = top_docs
            .par_iter()
            .filter(|(score, _)| score >= &(opts.threshold as f32))
            .map(|(_, doc_address)| -> Entry {
                let retrieved_doc = searcher.doc(*doc_address).unwrap();

                let bytes = retrieved_doc
                    .get_first(*FIELD_BUFFER)
                    .unwrap()
                    .as_bytes()
                    .unwrap();

                let archive = unsafe { archived_root::<Entry>(&bytes[..]) };

                archive.to_entry().unwrap()
            })
            .collect();

        Ok(entries)
    }
}
