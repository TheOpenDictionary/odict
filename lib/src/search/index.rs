use std::{error::Error, ffi::OsStr, fs::create_dir_all, fs::remove_dir_all, path::PathBuf};

use tantivy::{doc, Index};

use crate::config::get_config_dir;
use crate::{Dictionary, PreviewOptions};

use super::constants::CHARABIA;
use super::schema::{FIELD_BUFFER, FIELD_DEFINITIONS, FIELD_TERM, SCHEMA};
use super::tokenizer::CharabiaTokenizer;

pub struct IndexOptions {
    pub memory: usize,
    pub dir: PathBuf,
    pub overwrite: bool,
    pub cb_on_item: Box<dyn Fn(usize, &str) + Send + Sync>,
}

pub fn get_default_index_dir() -> PathBuf {
    get_config_dir().unwrap().join(".idx")
}

impl IndexOptions {
    pub fn default() -> Self {
        Self {
            memory: 50_000_000,
            overwrite: false,
            dir: get_default_index_dir(),
            cb_on_item: Box::new(|_, _| {}),
        }
    }

    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    pub fn memory(mut self, memory: usize) -> Self {
        self.memory = memory;
        self
    }

    pub fn dir<P: AsRef<OsStr> + ?Sized>(mut self, dir: &P) -> Self {
        self.dir = PathBuf::from(dir);
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

impl AsRef<IndexOptions> for IndexOptions {
    fn as_ref(&self) -> &IndexOptions {
        self
    }
}

impl Dictionary {
    pub fn index<Options: AsRef<IndexOptions>>(
        &self,
        options: Options,
    ) -> Result<(), Box<dyn Error>> {
        let opts = options.as_ref();
        let index_path = opts.dir.join(self.id.as_str());

        if opts.overwrite && index_path.exists() {
            remove_dir_all(&index_path)?;
        }

        if !index_path.exists() {
            create_dir_all(&index_path)?;
        }

        let index = Index::create_in_dir(&index_path, SCHEMA.to_owned())?;

        #[cfg(feature = "charabia")]
        index
            .tokenizers()
            .register(CHARABIA, CharabiaTokenizer::default());

        let mut index_writer = index.writer(opts.memory)?;

        self.entries.values().enumerate().for_each(|(i, entry)| {
            let document = doc!(
              *FIELD_TERM => entry.term.as_str(),
              *FIELD_DEFINITIONS => entry.preview(PreviewOptions::default()),
              *FIELD_BUFFER => entry.serialize().unwrap()
            );

            if index_writer.add_document(document).is_ok() {
                let cb = opts.cb_on_item.as_ref();
                cb(i, entry.term.as_str());
            }
        });

        index_writer.commit()?;
        index_writer.wait_merging_threads()?;

        Ok(())
    }
}
