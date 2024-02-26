use std::error::Error;

use rayon::iter::{ParallelBridge, ParallelIterator};
use tantivy::{doc, Index};

use crate::config::get_config_dir;
use crate::{Dictionary, PreviewOptions};

use super::schema::{FIELD_DEFINITIONS, FIELD_TERM, SCHEMA};

pub struct IndexOptions {
    batch_size: usize,
    cb_on_item: Box<dyn Fn(usize, &str) + Send + Sync>,
}

impl IndexOptions {
    pub fn default() -> Self {
        Self {
            batch_size: 10_000,
            cb_on_item: Box::new(|_, _| {}),
        }
    }

    pub fn batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    pub fn on_item<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize, &str) + Send + Sync + 'static,
    {
        self.cb_on_item = Box::new(callback);
        self
    }
}

impl Dictionary {
    pub fn index(&self, options: &IndexOptions) -> Result<(), Box<dyn Error>> {
        let index_path = get_config_dir()?.join(".idx").join(self.id.as_str());
        let index = Index::create_in_dir(&index_path, SCHEMA.to_owned())?;

        let mut index_writer = index.writer(options.batch_size)?;

        self.entries
            .values()
            .enumerate()
            .par_bridge()
            .for_each(|(i, entry)| {
                let document = doc!(
                  *FIELD_TERM => entry.term.as_str(),
                  *FIELD_DEFINITIONS => entry.preview(PreviewOptions::default())
                );

                if index_writer.add_document(document).is_ok() {
                    let cb = options.cb_on_item.as_ref();
                    cb(i, entry.term.as_str());
                }
            });

        index_writer.commit()?;
        index_writer.wait_merging_threads()?;

        Ok(())
    }
}
