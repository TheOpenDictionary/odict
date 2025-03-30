// use std::{borrow::BorrowMut, path::PathBuf, vec};

// use napi::{JsArrayBufferValue, bindgen_prelude::*};

// use merge::Merge;
// use odict::ToDictionary;

// use crate::{
//   types::{
//     self, DictionaryOptions, Entry, IndexOptions, LookupOptions, LookupQuery, SearchOptions,
//     SplitOptions,
//   },
//   utils::{cast_error, resolve_options, to_lookup_query},
// };

#[napi]
pub struct Dictionary {
  // options: Option<DictionaryOptions>,
  // file: odict::DictionaryFile,
}

// #[napi]
// pub fn compile(xml: String) -> Result<Vec<u8>> {
//   let dictionary = xml.to_dictionary().map_err(cast_error)?;

//   let bytes = odict::DictionaryWriter::default()
//     .write_to_bytes(&dictionary)
//     .map_err(cast_error)?;

//   Ok(bytes)
// }

#[napi]
impl Dictionary {
  // #[napi(constructor)]
  // pub fn new(data: &[u8], options: Option<DictionaryOptions>) -> Result<Self> {
  //   let reader = odict::DictionaryReader::default();
  //   let file = reader.read_from_bytes(data).map_err(cast_error)?;
  //   let dict = Dictionary { options, file };

  //   Ok(dict)
  // }

  //   pub fn options(&self) -> DictionaryOptions {
  //     resolve_options(&self.options)
  //   }

  //   pub fn _lookup(
  //     &self,
  //     queries: &Vec<odict::lookup::LookupQuery>,
  //     options: Option<LookupOptions>,
  //   ) -> Result<Vec<Vec<Entry>>> {
  //     let dict = self.file.to_archive().map_err(cast_error)?;

  //     let mut opts = options.unwrap_or(LookupOptions::default());

  //     if let Some(split) = opts
  //       .split
  //       .or(self.options().split.map(|s| s.threshold).flatten())
  //     {
  //       opts.split = Some(split);
  //     }

  //     let entries = dict
  //       .lookup::<odict::lookup::LookupQuery, &odict::lookup::LookupOptions>(queries, &opts.into())
  //       .map_err(|e| cast_error(e))?;

  //     let mapped = entries
  //       .iter()
  //       .map(|i| {
  //         i.iter()
  //           .map(|e| Entry::from_archive(e))
  //           .collect::<Result<Vec<Entry>, _>>()
  //       })
  //       .collect::<Result<Vec<Vec<Entry>>, _>>()?;

  //     Ok(mapped)
  //   }

  //   #[napi]
  //   pub fn lookup(
  //     &self,
  //     query: Either3<types::LookupQuery, String, Vec<Either<LookupQuery, String>>>,
  //     options: Option<LookupOptions>,
  //   ) -> Result<Vec<Vec<Entry>>> {
  //     let mut queries: Vec<odict::lookup::LookupQuery> = vec![];

  //     match query {
  //       Either3::A(a) => queries.push(a.into()),
  //       Either3::B(b) => queries.push(b.into()),
  //       Either3::C(c) => queries.append(
  //         c.into_iter()
  //           .map(to_lookup_query)
  //           .collect::<Vec<odict::lookup::LookupQuery>>()
  //           .borrow_mut(),
  //       ),
  //     }

  //     self._lookup(&queries, options)
  //   }

  //   #[napi]
  //   pub fn lexicon(&self) -> Result<Vec<&str>> {
  //     let dict = self.file.to_archive().map_err(cast_error)?;
  //     let lexicon = dict.lexicon();

  //     Ok(lexicon)
  //   }

  //   #[napi]
  //   pub fn split(&self, query: String, options: Option<SplitOptions>) -> Result<Vec<Entry>> {
  //     let dict = self.file.to_archive().map_err(cast_error)?;

  //     let mut opts = options;

  //     opts.merge(self.options().split);

  //     let result = dict
  //       .split::<&odict::split::SplitOptions>(&query, &opts.unwrap().into())
  //       .map_err(|e| cast_error(e))?;

  //     Ok(
  //       result
  //         .iter()
  //         .map(|e| Entry::from_archive(e))
  //         .collect::<Result<Vec<Entry>, _>>()?,
  //     )
  //   }

  //   #[napi]
  //   pub fn index(&self, options: Option<IndexOptions>) -> Result<()> {
  //     #[cfg(feature = "search")]
  //     {
  //       let dict = self.file.to_archive().map_err(cast_error)?;
  //       let mut opts = options;

  //       opts.merge(self.options().index);

  //       dict
  //         .index::<&odict::search::IndexOptions>(&opts.unwrap().into())
  //         .map_err(cast_error)?;

  //       return Ok(());
  //     }

  //     unimplemented!("index() is not available in browser environments. Maybe try IndexedDB?");
  //   }

  //   #[napi]
  //   pub fn search(&self, query: String, options: Option<SearchOptions>) -> Result<Vec<Entry>> {
  //     #[cfg(feature = "search")]
  //     {
  //       let dict = self.file.to_archive().map_err(cast_error)?;
  //       let mut opts = options;

  //       opts.merge(self.options().search);

  //       let results = dict
  //         .search::<&odict::search::SearchOptions>(query.as_str(), &opts.unwrap().into())
  //         .map_err(cast_error)?;

  //       let entries = results
  //         .iter()
  //         .map(|e| Entry::from_entry(e.clone()))
  //         .collect::<Result<Vec<Entry>, _>>()?;

  //       return Ok(entries);
  //     }

  //     unimplemented!("search() is not available in browser environments. Maybe try IndexedDB?");
  //   }
}

// #[cfg(test)]
// mod test {
//   use merge::Merge;

//   #[test]
//   fn test_options_merging() {
//     let opts1 = crate::types::DictionaryOptions {
//       search: None,
//       index: Some(crate::types::IndexOptions {
//         directory: Some("test".to_string()),
//         memory: Some(1234),
//         overwrite: Some(false),
//       }),
//       split: { Some(crate::types::SplitOptions { threshold: Some(5) }) },
//     };

//     let mut opts2: Option<crate::types::IndexOptions> = None;

//     let mut opts3: Option<crate::types::SplitOptions> = Some(crate::types::SplitOptions {
//       threshold: Some(10),
//     });

//     opts2.merge(opts1.index);
//     opts3.merge(opts1.split);

//     let result1 = opts2.unwrap();
//     let result2 = opts3.unwrap();

//     assert_eq!(result1.directory.unwrap(), "test".to_string());
//     assert_eq!(result1.memory.unwrap(), 1234);
//     assert_eq!(result1.overwrite.unwrap(), false);
//     assert_eq!(result2.threshold.unwrap(), 10);
//   }
// }
