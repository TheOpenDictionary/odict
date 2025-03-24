use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct SearchOptions {
    pub directory: Option<String>,
    pub threshold: Option<u32>,
    pub autoindex: Option<bool>,
    pub limit: Option<u32>,
}

impl Into<odict::search::SearchOptions> for SearchOptions {
    fn into(self) -> odict::search::SearchOptions {
        odict::search::SearchOptions {
            dir: self.directory.map(std::path::PathBuf::from),
            threshold: self.threshold.unwrap_or(80),
            autoindex: self.autoindex.unwrap_or(true),
            limit: self.limit.map(|l| l as usize).unwrap_or(10),
            tokenizer: odict::search::TextAnalyzer::default(),
        }
    }
}
