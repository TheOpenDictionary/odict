use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct SplitOptions {
    pub threshold: Option<u32>,
}

impl Into<odict::core::split::SplitOptions> for SplitOptions {
    fn into(self) -> odict::core::split::SplitOptions {
        odict::core::split::SplitOptions {
            threshold: self.threshold.unwrap_or(1000) as usize,
        }
    }
}
