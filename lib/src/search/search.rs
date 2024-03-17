use std::{error::Error, ffi::OsStr, path::PathBuf};

use tantivy::{
    collector::TopDocs, query::QueryParser, schema::TantivyDocument, Document, Index, ReloadPolicy,
};

use crate::Dictionary;

use super::{
    get_default_index_dir,
    schema::{FIELD_DEFINITIONS, FIELD_TERM, SCHEMA},
};

pub struct SearchOptions {
    pub dir: PathBuf,
    pub exact: bool,
}

impl SearchOptions {
    pub fn default() -> Self {
        Self {
            dir: get_default_index_dir(),
            exact: false,
        }
    }

    pub fn dir<P: AsRef<OsStr> + ?Sized>(mut self, dir: &P) -> Self {
        self.dir = PathBuf::from(dir);
        self
    }

    pub fn exact(mut self, exact: bool) -> Self {
        self.exact = exact;
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
    ) -> Result<(), Box<dyn Error>> {
        let opts = options.as_ref();
        let index_path = opts.dir.join(self.id.as_str());
        let index = Index::create_in_dir(&index_path, SCHEMA.to_owned())?;

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()?;

        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(&index, vec![*FIELD_TERM, *FIELD_DEFINITIONS]);
        let query_obj = query_parser.parse_query(query)?;
        let top_docs = searcher.search(&query_obj, &TopDocs::with_limit(10))?;

        for (_score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address)?;
            // println!("{}", retrieved_doc.to_json(&schema));
        }

        Ok(())
    }
}
