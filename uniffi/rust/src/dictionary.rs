use uniffi::export;
use std::sync::{Arc, RwLock};
use odict::ToDictionary;
use crate::types::{LoadOptions, LookupResult, SearchOptions, Entry, Token, LookupOptions, IndexOptions};
use crate::utils::Error;
use odict::format::json::ToJSON;
use odict::format::html::ToHTML;
use odict::format::md::ToMarkdown;
use odict::schema::ArchivedEntry;

#[export]
pub fn compile(xml: String) -> Result<Vec<u8>, Error> {
    let bytes = xml
        .to_dictionary()
        .and_then(|d| d.build())
        .and_then(|d| d.to_bytes())
        .map_err(Error::from)?;
    Ok(bytes)
}

#[export]
pub async fn load_dictionary(
    dictionary: String,
    options: Option<LoadOptions>,
) -> Result<Arc<OpenDictionary>, Error> {
    let dict = match options {
        Some(opts) => {
            let load_opts = opts.try_into().map_err(Error::from)?;
            odict::OpenDictionary::load_with_options(&dictionary, load_opts)
                .await
                .map_err(Error::from)?
        }
        None => odict::OpenDictionary::load(&dictionary)
            .await
            .map_err(Error::from)?,
    };

    Ok(Arc::new(OpenDictionary { dict: RwLock::new(dict) }))
}

#[export]
pub fn dictionary_from_bytes(bytes: Vec<u8>) -> Result<Arc<OpenDictionary>, Error> {
    let dict = odict::OpenDictionary::from_bytes(bytes).map_err(Error::from)?;
    Ok(Arc::new(OpenDictionary { dict: RwLock::new(dict) }))
}

#[export]
pub fn dictionary_from_xml(xml: String) -> Result<Arc<OpenDictionary>, Error> {
    let bytes = compile(xml)?;
    dictionary_from_bytes(bytes)
}

#[derive(uniffi::Object)]
pub struct OpenDictionary {
    pub(crate) dict: RwLock<odict::OpenDictionary>,
}

#[export]
impl OpenDictionary {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let dict = self.dict.read().map_err(|e| Error::Odict(format!("Lock error: {}", e)))?;
        dict.to_bytes().map_err(Error::from)
    }

    pub fn to_bytes_with_options(
        &self,
        quality: Option<u32>,
        window_size: Option<u32>,
    ) -> Result<Vec<u8>, Error> {
        let dict = self.dict.read().map_err(|e| Error::Odict(format!("Lock error: {}", e)))?;
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

            dict.to_bytes_with_options(compiler_options)
                .map_err(Error::from)
        } else {
            dict.to_bytes().map_err(Error::from)
        }
    }

    pub fn save(
        &self,
        path: String,
        quality: Option<u32>,
        window_size: Option<u32>,
    ) -> Result<(), Error> {
        let mut dict = self.dict.write().map_err(|e| Error::Odict(format!("Lock error: {}", e)))?;
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

            dict.to_disk_with_options(&path, compiler_options)
                .map_err(Error::from)
        } else {
            dict.to_disk(&path).map_err(Error::from)
        }
    }

    pub fn min_rank(&self) -> Result<Option<u32>, Error> {
        let dict = self.dict.read().map_err(|e| Error::Odict(format!("Lock error: {}", e)))?;
        Ok(dict.contents().map_err(Error::from)?.min_rank())
    }

    pub fn max_rank(&self) -> Result<Option<u32>, Error> {
        let dict = self.dict.read().map_err(|e| Error::Odict(format!("Lock error: {}", e)))?;
        Ok(dict.contents().map_err(Error::from)?.max_rank())
    }

    pub fn lookup(
        &self,
        queries: Vec<String>,
        options: Option<LookupOptions>,
    ) -> Result<Vec<LookupResult>, Error> {
        let options = options.unwrap_or_default();
        let dict_guard = self.dict.read().map_err(|e| Error::Odict(format!("Lock error: {}", e)))?;
        let dict = dict_guard.contents().map_err(Error::from)?;

        let results = dict
            .lookup(&queries, &odict::lookup::LookupOptions::from(options))
            .map_err(Error::from)?;

        let mapped = results
            .iter()
            .map(LookupResult::from_archive)
            .collect::<Result<Vec<LookupResult>, _>>()?;

        Ok(mapped)
    }

    pub fn lexicon(&self) -> Result<Vec<String>, Error> {
        let dict_guard = self.dict.read().map_err(|e| Error::Odict(format!("Lock error: {}", e)))?;
        let dict = dict_guard.contents().map_err(Error::from)?;
        let lexicon = dict.lexicon().into_iter().map(|s| s.to_string()).collect();

        Ok(lexicon)
    }

    pub fn index(&self, options: Option<IndexOptions>) -> Result<(), Error> {
        let dict_guard = self.dict.read().map_err(|e| Error::Odict(format!("Lock error: {}", e)))?;
        let dict = dict_guard.contents().map_err(Error::from)?;
        let opts = options.unwrap_or_default();

        dict.index(odict::index::IndexOptions::from(opts))
            .map_err(Error::from)?;

        Ok(())
    }

    pub fn search(&self, query: String, options: Option<SearchOptions>) -> Result<Vec<Entry>, Error> {
        let dict_guard = self.dict.read().map_err(|e| Error::Odict(format!("Lock error: {}", e)))?;
        let dict = dict_guard.contents().map_err(Error::from)?;
        let opts = options.unwrap_or_default();

        let results = dict
            .search(query.as_str(), odict::search::SearchOptions::from(opts))
            .map_err(Error::from)?;

        let entries = results
            .into_iter()
            .map(|e| e.into())
            .collect::<Vec<Entry>>();

        Ok(entries)
    }

    pub fn tokenize(
        &self,
        text: String,
        follow: Option<bool>,
        insensitive: Option<bool>,
    ) -> Result<Vec<Token>, Error> {
        let dict_guard = self.dict.read().map_err(|e| Error::Odict(format!("Lock error: {}", e)))?;
        let dict = dict_guard.contents().map_err(Error::from)?;

        let mut opts = odict::tokenize::TokenizeOptions::default();

        if let Some(f) = follow {
            opts = opts.follow(f);
        }

        if let Some(i) = insensitive {
            opts = opts.insensitive(i);
        }

        let tokens = dict.tokenize(&text, opts).map_err(Error::from)?;

        let mut mapped_tokens = Vec::new();

        for token in tokens {
            let mut entries = Vec::new();

            for result in &token.entries {
                let entry: Entry = result.entry.deserialize().map_err(Error::from)?.into();

                let directed_from = match result.directed_from {
                    Some(entry) => Some(entry.deserialize().map_err(Error::from)?.into()),
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
                start: token.start as u32,
                end: token.end as u32,
                entries,
            });
        }

        Ok(mapped_tokens)
    }

    pub fn to_json(&self, pretty: bool) -> Result<String, Error> {
        let dict_guard = self.dict.read().map_err(|e| Error::Odict(format!("Lock error: {}", e)))?;
        let dict = dict_guard.contents().map_err(Error::from)?.deserialize().map_err(Error::from)?;
        dict.to_json(pretty).map_err(Error::from)
    }

    pub fn to_html(&self) -> Result<String, Error> {
        let dict_guard = self.dict.read().map_err(|e| Error::Odict(format!("Lock error: {}", e)))?;
        let dict = dict_guard.contents().map_err(Error::from)?;
        let mut html = String::new();
        for entry in dict.entries.iter() {
            let entry_ref: &ArchivedEntry = entry;
            html.push_str(&ToHTML::to_html(entry_ref).map_err(Error::from)?);
        }
        Ok(html)
    }

    pub fn to_markdown(&self) -> Result<String, Error> {
        let dict_guard = self.dict.read().map_err(|e| Error::Odict(format!("Lock error: {}", e)))?;
        let dict = dict_guard.contents().map_err(Error::from)?;
        let mut md = String::new();
        for entry in dict.entries.iter() {
            let entry_ref: &ArchivedEntry = entry;
            md.push_str(&ToMarkdown::to_markdown(entry_ref).map_err(Error::from)?);
        }
        Ok(md)
    }
}
