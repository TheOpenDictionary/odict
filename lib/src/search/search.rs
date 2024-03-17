use std::{error::Error, ffi::OsStr, path::PathBuf};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rkyv::{archived_root, Deserialize, Infallible};
use tantivy::{collector::TopDocs, query::QueryParser, Index, ReloadPolicy};

use crate::{Dictionary, Entry};

use super::{
    get_default_index_dir,
    schema::{FIELD_BUFFER, FIELD_DEFINITIONS, FIELD_TERM},
};

pub struct SearchOptions {
    pub dir: PathBuf,
}

impl SearchOptions {
    pub fn default() -> Self {
        Self {
            dir: get_default_index_dir(),
        }
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
            .map(|(score, doc_address)| -> Entry {
                let retrieved_doc = searcher.doc(*doc_address).unwrap();

                let bytes = retrieved_doc
                    .get_first(*FIELD_BUFFER)
                    .unwrap()
                    .as_bytes()
                    .unwrap();

                let archive = unsafe { archived_root::<Entry>(&bytes[..]) };
                let entry: Entry = archive.deserialize(&mut Infallible).unwrap();
                println!("{} {}", score, entry.clone().term);
                entry
            })
            .collect();

        Ok(entries)
    }
}
