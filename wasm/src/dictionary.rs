
use wasm_bindgen::prelude::*;

use crate::{utils::{cast_error, to_lookup_query}, Either3};

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

  pub fn _lookup(
    &self,
    queries: &Vec<odict::lookup::LookupQuery>,
    options: Option<LookupOptions>,
  ) -> Result<Vec<Vec<Entry>>> {
    let dict = self.file.to_archive().map_err(cast_error)?;

    let mut opts = options.unwrap_or(LookupOptions::default());

    if let Some(split) = opts
      .split
      .or(self.options().split.map(|s| s.threshold).flatten())
    {
      opts.split = Some(split);
    }

    let entries = dict
      .lookup::<odict::lookup::LookupQuery, &odict::lookup::LookupOptions>(queries, &opts.into())
      .map_err(|e| cast_error(e))?;

    let mapped = entries
      .iter()
      .map(|i| {
        i.iter()
          .map(|e| Entry::from_archive(e))
          .collect::<Result<Vec<Entry>, _>>()
      })
      .collect::<Result<Vec<Vec<Entry>>, _>>()?;

    Ok(mapped)
  }

  #[wasm_bindgen]
  pub fn lookup(
    &self,
    query: JsValue,
    options: Option<LookupOptions>,
  ) -> Result<Vec<Vec<Entry>>, JsValue> {
    let mut queries: Vec<odict::lookup::LookupQuery> = vec![];
    let queryUnion: Either3<crate::LookupQuery, String, Vec<crate::LookupQuery>> = serde_wasm_bindgen::from_value(query)?;

    match queryUnion {
      Either3::A(a) => queries.push(a.into()),
      Either3::B(b) => queries.push(b.into()),
      Either3::C(c) => queries.append(
        c.into_iter()
          .map(to_lookup_query)
          .collect::<Vec<odict::lookup::LookupQuery>>()
          .borrow_mut(),
      ),
    }

    self._lookup(&queries, options)
  }
}
