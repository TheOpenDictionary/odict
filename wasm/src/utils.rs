use merge::Merge;
use wasm_bindgen::JsValue;

use crate::LookupQuery;

// use crate::{DictionaryOptions, LookupQuery};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn cast_error<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&format!("{}", err))
}

// pub fn resolve_options(options: &Option<DictionaryOptions>) -> DictionaryOptions {
//   match options {
//     Some(opts) => {
//       let mut out = opts.clone();
//       out.merge(DictionaryOptions::default());
//       return out;
//     }
//     None => DictionaryOptions::default(),
//   }
// }

pub fn to_lookup_query(value: JsValue) -> odict::lookup::LookupQuery {
    if let Some(s) = value.as_string() {
        s.into()
    } else if let Ok(lq) = serde_wasm_bindgen::from_value::<LookupQuery>(value) {
        lq.into()
    } else {
        // Default to empty string if conversion fails
        "".to_string().into()
    }
}
