use std::{error::Error, ffi::OsStr, fs::create_dir_all, fs::remove_dir_all, path::PathBuf};

use tantivy::{doc, Index};

use crate::config::get_config_dir;
use crate::{ArchivedDictionary, Dictionary, PreviewOptions};

use super::schema::{FIELD_DEFINITIONS, FIELD_TERM, SCHEMA};

pub struct IndexOptions {
    memory: usize,
    dir: PathBuf,
    overwrite: bool,
    cb_on_item: Box<dyn Fn(usize, &str) + Send + Sync>,
}

impl IndexOptions {
    pub fn default() -> Self {
        Self {
            memory: 50_000_000,
            overwrite: false,
            dir: get_config_dir().unwrap().join(".idx"),
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

macro_rules! index {
	($t:ident) => {
		impl $t {
		    pub fn index<Options: AsRef<IndexOptions>>(&self, options: Options) -> Result<(), Box<dyn Error>> {
				let opts = options.as_ref();
		        let index_path = opts.dir.join(self.id.as_str());

				if opts.overwrite && index_path.exists() {
					remove_dir_all(&index_path)?;
				}

				if !index_path.exists() {
		            create_dir_all(&index_path)?;
		        }

		        let index = Index::create_in_dir(&index_path, SCHEMA.to_owned())?;

		        let mut index_writer = index.writer(opts.memory)?;

		        self.entries.values().enumerate().for_each(|(i, entry)| {
		            let document = doc!(
		              *FIELD_TERM => entry.term.as_str(),
		              *FIELD_DEFINITIONS => entry.preview(PreviewOptions::default())
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
	};
}

index!(Dictionary);
index!(ArchivedDictionary);
