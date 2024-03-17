use std::{error::Error, ffi::OsStr, path::PathBuf};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rkyv::{archived_root, Deserialize, Infallible};
use tantivy::{collector::TopDocs, query::QueryParser, Index, ReloadPolicy};

use crate::{Dictionary, Entry};

use super::{
    constants::CHARABIA,
    get_default_index_dir,
    schema::{FIELD_BUFFER, FIELD_DEFINITIONS, FIELD_TERM},
    tokenizer::CharabiaTokenizer,
};

pub struct SearchOptions {
    pub dir: PathBuf,
    pub threshold: u32,
}

impl SearchOptions {
    pub fn default() -> Self {
        Self {
            dir: get_default_index_dir(),
            threshold: 1,
        }
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
            .register(CHARABIA, CharabiaTokenizer::default());

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()?;

        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(&index, vec![*FIELD_TERM, *FIELD_DEFINITIONS]);
        let query_obj = query_parser.parse_query(query)?;
        let top_docs = searcher.search(&query_obj, &TopDocs::with_limit(10))?;
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
                let entry: Entry = archive.deserialize(&mut Infallible).unwrap();

                entry
            })
            .collect();

        Ok(entries)
    }
}
