use napi::bindgen_prelude::*;

use crate::types::LoadOptions;
use crate::types::SaveOptions;
use crate::{
    shared::compile,
    types::{self, Entry},
    utils::cast_error,
};

#[napi]
pub struct OpenDictionary {
    dict: odict::OpenDictionary,
}

#[napi]
impl OpenDictionary {
    #[napi(constructor)]
    pub fn new(data: Either<Buffer, String>) -> Result<Self> {
        let bytes = match data {
            Either::A(buffer) => buffer,
            Either::B(xml) => compile(xml)?,
        };

        let dict = crate::shared::new_from_bytes(&bytes)?;

        Ok(Self { dict })
    }

    #[napi(getter)]
    pub fn bytes(&self) -> Result<Vec<u8>> {
        Ok(self.dict.to_bytes().map_err(cast_error)?)
    }

    #[napi(getter)]
    pub fn min_rank(&self) -> Result<Option<u32>> {
        crate::shared::get_min_rank(&self.dict)
    }

    #[napi(getter)]
    pub fn max_rank(&self) -> Result<Option<u32>> {
        crate::shared::get_max_rank(&self.dict)
    }

    #[napi(factory)]
    pub async fn load(dictionary: String, options: Option<LoadOptions>) -> Result<Self> {
        let dict = match options {
            Some(opts) => {
                let load_opts = odict::LoadOptions::try_from(opts).map_err(cast_error)?;
                odict::OpenDictionary::load_with_options(&dictionary, load_opts)
                    .await
                    .map_err(cast_error)?
            }
            None => odict::OpenDictionary::load(&dictionary)
                .await
                .map_err(cast_error)?,
        };

        Ok(Self { dict })
    }

    #[napi]
    pub fn save(&mut self, path: String, options: Option<SaveOptions>) -> Result<()> {
        match options {
            Some(opts) => {
                let compiler_options = odict::compile::CompilerOptions::from(opts);
                self.dict
                    .to_disk_with_options(&path, compiler_options)
                    .map_err(cast_error)
            }
            None => self.dict.to_disk(&path).map_err(cast_error),
        }
    }

    #[napi]
    pub fn lookup(
        &self,
        query: Either<String, Vec<String>>,
        options: Option<types::LookupOptions>,
    ) -> Result<Vec<types::LookupResult>> {
        crate::shared::perform_lookup(&self.dict, query, options)
    }

    #[napi]
    pub fn lexicon(&self) -> Result<Vec<&str>> {
        crate::shared::get_lexicon(&self.dict)
    }

    #[napi]
    pub fn index(&self, options: Option<types::IndexOptions>) -> Result<()> {
        let dict = self.dict.contents().map_err(cast_error)?;
        let opts = options.unwrap_or(types::IndexOptions::default());

        dict.index(odict::index::IndexOptions::from(opts))
            .map_err(cast_error)?;

        Ok(())
    }

    #[napi]
    pub fn search(
        &self,
        query: String,
        options: Option<types::SearchOptions>,
    ) -> Result<Vec<Entry>> {
        let dict = self.dict.contents().map_err(cast_error)?;
        let opts = options.unwrap_or_default();

        // Use our helper function to avoid orphan rule issues
        let results = dict
            .search(query.as_str(), odict::search::SearchOptions::from(opts))
            .map_err(cast_error)?;

        // Use the new from_entry function for Entry types
        let entries = results
            .iter()
            .map(|e| e.clone().into())
            .collect::<Vec<Entry>>();

        Ok(entries)
    }

    #[napi]
    pub fn tokenize(
        &self,
        text: String,
        options: Option<types::TokenizeOptions>,
    ) -> Result<Vec<types::Token>> {
        crate::shared::perform_tokenization(&self.dict, &text, options)
    }
}
