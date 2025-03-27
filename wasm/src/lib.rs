mod dictionary;
mod types;
mod utils;

use wasm_bindgen::prelude::*;

// Re-export the Dictionary and related types
pub use dictionary::Dictionary;
pub use types::*;

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
}
