use std::error::Error;

use rayon::iter::{ParallelBridge, ParallelIterator};
use tantivy::schema::Schema;
use tantivy::{doc, Index};

use crate::config::get_config_dir;
use crate::{Dictionary, PreviewOptions};

use super::schema::{FIELD_DEFINITIONS, FIELD_TERM, SCHEMA};

pub struct IndexOptions {
    batch_size: usize,
}

impl IndexOptions {
    pub fn default() -> Self {
        Self { batch_size: 10_000 }
    }

    pub fn batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }
}

impl Dictionary {
    pub fn index(&self, options: &IndexOptions) -> Result<(), Box<dyn Error>> {
        let index_path = get_config_dir()?.join(".idx").join(self.id.as_str());
        let index = Index::create_in_dir(&index_path, SCHEMA.to_owned())?;

        let mut schema_builder = Schema::builder();
        let mut index_writer = index.writer(options.batch_size)?;

        self.entries.values().par_bridge().for_each(|entry| {
            let mut document = doc!(
              *FIELD_TERM => entry.term.as_str(),
              *FIELD_DEFINITIONS => entry.preview(PreviewOptions::default())
            );
        });

        // schema_builder.
        let schema = schema_builder.build();

        Ok(())
    }
}
