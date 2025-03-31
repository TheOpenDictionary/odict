use crate::{ArchivedDictionary, ArchivedEntry, Dictionary, Entry};

use rayon::prelude::*;
use rkyv::option::ArchivedOption;
use std::marker::{Send, Sync};

mod options;

pub use options::*;

pub struct LookupResult<E> {
    pub entry: E,
    pub directed_from: Option<E>,
}

/* -------------------------------------------------------------------------- */
/*                                   Methods                                  */
/* -------------------------------------------------------------------------- */

macro_rules! lookup {
    ($tys:ident, $ret:ident, $opt:ident) => {
        impl $tys {
            fn find_entry<'a>(
                &'a self,
                follow: &bool,
                query: &str,
                directed_from: Option<&'a $ret>,
            ) -> $opt<LookupResult<&'a $ret>> {
                if let Some(entry) = self.entries.get(query) {
                    // Follow an alias if it exists
                    if *follow {
                        if let $opt::Some(also) = &entry.see_also {
                            return self.find_entry(follow, also, Some(entry));
                        }
                    }

                    return $opt::Some(LookupResult {
                        entry,
                        directed_from,
                    });
                }

                $opt::None
            }

            fn perform_lookup<'a, Options>(
                &'a self,
                query: &'a str,
                options: Options,
            ) -> crate::Result<Vec<LookupResult<&'a $ret>>>
            where
                Options: AsRef<LookupOptions> + Send + Sync,
            {
                let LookupOptions { follow, strategy } = options.as_ref();

                if let $opt::Some(result) = self.find_entry(follow, query, None) {
                    return Ok(vec![result]);
                }

                let mut results: Vec<LookupResult<&$ret>> = Vec::new();

                if let LookupStrategy::Split(min_length) = strategy {
                    let chars: Vec<_> = query.chars().collect();
                    let mut start = 0;
                    let mut end = chars.len();

                    while start < end {
                        let substr: String = chars[start..end].iter().collect();
                        let maybe_entry = self.find_entry(follow, substr.as_str(), None);

                        if maybe_entry.is_some() || substr.len() <= *min_length {
                            start = end;
                            end = chars.len();

                            if let $opt::Some(result) = maybe_entry {
                                results.push(result);
                            }

                            continue;
                        }

                        end -= 1;
                    }
                }

                Ok(results)
            }

            pub fn lookup<'a, Query, Options>(
                &'a self,
                queries: &'a Vec<Query>,
                options: Options,
            ) -> crate::Result<Vec<LookupResult<&'a $ret>>>
            where
                Query: AsRef<str> + Send + Sync,
                Options: AsRef<LookupOptions> + Send + Sync,
            {
                let results = queries
                    .par_iter()
                    .map(|query| self.perform_lookup(query.as_ref(), &options))
                    .collect::<crate::Result<Vec<_>>>()?
                    .into_iter()
                    .flatten()
                    .collect::<Vec<LookupResult<&'a $ret>>>();

                Ok(results)
            }
        }
    };
}

lookup!(Dictionary, Entry, Option);
lookup!(ArchivedDictionary, ArchivedEntry, ArchivedOption);
