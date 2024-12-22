use std::{borrow::BorrowMut, path::PathBuf};

use either::Either;
use odict::lookup::LookupOptions;
use pyo3::prelude::*;

use crate::{types::Entry, utils::cast_error};

fn lookup(
    file: &odict::DictionaryFile,
    queries: &Vec<odict::lookup::LookupQuery>,
    split: Option<usize>,
    follow: Option<bool>,
) -> PyResult<Vec<Vec<Entry>>> {
    let dict = file.to_archive().map_err(cast_error)?;

    let mut opts = LookupOptions::default();

    if let Some(split) = split {
        opts.split = split;
    }

    if let Some(follow) = follow {
        opts.follow = follow;
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

#[pyclass]
pub struct Dictionary {
    file: odict::DictionaryFile,
}

#[pymethods]
impl Dictionary {
    #[new]
    pub fn new(path_or_alias: String) -> PyResult<Self> {
        let reader = odict::DictionaryReader::default();

        let file = reader
            .read_from_path_or_alias(&path_or_alias)
            .map_err(cast_error)?;

        let dict = Dictionary { file };

        Ok(dict)
    }

    #[staticmethod]
    pub fn write(xml_str: String, out_path: String) -> PyResult<Self> {
        let dict = odict::Dictionary::from(&xml_str).map_err(cast_error)?;
        let reader = odict::DictionaryReader::default();
        let writer = odict::DictionaryWriter::default();

        writer.write_to_path(&dict, &out_path).map_err(cast_error)?;

        let file = reader.read_from_path(&out_path).map_err(cast_error)?;

        let dict = Dictionary { file };

        Ok(dict)
    }

    #[staticmethod]
    #[pyo3(signature = (xml_path, out_path=None))]
    pub fn compile(xml_path: String, out_path: Option<String>) -> PyResult<Self> {
        let in_file = PathBuf::from(xml_path.to_owned());

        let out_file = out_path.unwrap_or_else(|| {
            odict::fs::infer_path(&xml_path)
                .to_string_lossy()
                .to_string()
        });

        let reader = odict::DictionaryReader::default();
        let writer = odict::DictionaryWriter::default();

        writer
            .compile_xml(&in_file, &out_file)
            .map_err(cast_error)?;

        let file = reader.read_from_path(&out_file).map_err(cast_error)?;

        let dict = Dictionary { file };

        Ok(dict)
    }

    // pub fn options(&self) -> DictionaryOptions {
    //     resolve_options(&self.options)
    // }

    #[getter]
    pub fn path(&self) -> PyResult<String> {
        let path = self
            .file
            .path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap();

        Ok(path)
    }

    #[pyo3(signature = (query, split=None, follow=None))]
    pub fn lookup(
        &self,
        query: Either<String, Vec<String>>,
        split: Option<usize>,
        follow: Option<bool>,
    ) -> PyResult<Vec<Vec<Entry>>> {
        let mut queries: Vec<odict::lookup::LookupQuery> = vec![];

        match query {
            Either::Left(a) => queries.push(a.into()),
            Either::Right(c) => queries.append(
                c.into_iter()
                    .map(|e| e.into())
                    .collect::<Vec<odict::lookup::LookupQuery>>()
                    .borrow_mut(),
            ),
        }

        lookup(&(self.file), &queries, split, follow)
    }

    pub fn lexicon(&self) -> PyResult<Vec<&str>> {
        let dict = self.file.to_archive().map_err(cast_error)?;
        let lexicon = dict.lexicon();

        Ok(lexicon)
    }

    // #[pyo3(signature = (env, query, options=None))]
    // pub fn split(
    //     &self,
    //     env: Env,
    //     query: String,
    //     options: Option<SplitOptions>,
    // ) -> PyResult<Vec<Entry>> {
    //     let dict = self.file.to_archive().map_err(cast_error)?;

    //     let mut opts = options;

    //     opts.merge(self.options().split);

    //     let result = dict
    //         .split::<&odict::split::SplitOptions>(&query, &opts.unwrap().into())
    //         .map_err(|e| cast_error(e))?;

    //     Ok(result
    //         .iter()
    //         .map(|e| Entry::from_archive(env, e))
    //         .collect::<Result<Vec<Entry>, _>>()?)
    // }

    // #[pyo3(signature = (options=None))]
    // pub fn index(&self, options: Option<IndexOptions>) -> PyResult<()> {
    //     let dict = self.file.to_archive().map_err(cast_error)?;
    //     let mut opts = options;

    //     opts.merge(self.options().index);

    //     dict.index::<&odict::search::IndexOptions>(&opts.unwrap().into())
    //         .map_err(cast_error)?;

    //     Ok(())
    // }

    // #[pyo3(signature = (env, query, options=None))]
    // pub fn search(
    //     &self,
    //     env: Env,
    //     query: String,
    //     options: Option<SearchOptions>,
    // ) -> PyResult<Vec<Entry>> {
    //     let dict = self.file.to_archive().map_err(cast_error)?;
    //     let mut opts = options;

    //     opts.merge(self.options().search);

    //     let results = dict
    //         .search::<&odict::search::SearchOptions>(query.as_str(), &opts.unwrap().into())
    //         .map_err(cast_error)?;

    //     let entries = results
    //         .iter()
    //         .map(|e| Entry::from_entry(env, e.clone()))
    //         .collect::<Result<Vec<Entry>, _>>()?;

    //     Ok(entries)
    // }
}

// #[cfg(test)]
// mod test {
//     use merge::Merge;

//     #[test]
//     fn test_options_merging() {
//         let opts1 = crate::types::DictionaryOptions {
//             search: None,
//             index: Some(crate::types::IndexOptions {
//                 directory: Some("test".to_string()),
//                 memory: Some(1234),
//                 overwrite: Some(false),
//             }),
//             split: { Some(crate::types::SplitOptions { threshold: Some(5) }) },
//         };

//         let mut opts2: Option<crate::types::IndexOptions> = None;

//         let mut opts3: Option<crate::types::SplitOptions> = Some(crate::types::SplitOptions {
//             threshold: Some(10),
//         });

//         opts2.merge(opts1.index);
//         opts3.merge(opts1.split);

//         let result1 = opts2.unwrap();
//         let result2 = opts3.unwrap();

//         assert_eq!(result1.directory.unwrap(), "test".to_string());
//         assert_eq!(result1.memory.unwrap(), 1234);
//         assert_eq!(result1.overwrite.unwrap(), false);
//         assert_eq!(result2.threshold.unwrap(), 10);
//     }
// }
