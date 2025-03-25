
use wasm_bindgen::prelude::*;

use crate::utils::cast_error;

#[wasm_bindgen]
pub struct Dictionary {
  // options: Option<DictionaryOptions>,
  file: odict::DictionaryFile,
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
impl Dictionary {
  #[wasm_bindgen(constructor)]
  pub fn new(bytes: &[u8],
      // options: Option<DictionaryOptions>
  ) -> Result<Self, JsValue> {
    let reader = odict::DictionaryReader::default();
log(format!("{} bytes", bytes.len()).as_str());
    let file = reader
        .read_from_bytes(&bytes)
        .map_err(cast_error)?;

    let dict = Dictionary { file };

    Ok(dict)
  }
}
