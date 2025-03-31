use std::{collections::HashMap, sync::LazyLock};

use self::options::{LookupOptions, LookupStrategy};
use crate::{ArchivedDictionary, ArchivedEntry, Dictionary, Entry};

use rayon::prelude::*;
use rkyv::option::ArchivedOption;
use std::marker::{Send, Sync};

use regex::Regex;

mod options;

const PARENTHETICAL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\((.+)\)$").unwrap());

struct LookupResult<E> {
    entry: E,
    directed_from: Option<E>,
}

/* -------------------------------------------------------------------------- */
/*                                   Methods                                  */
/* -------------------------------------------------------------------------- */

macro_rules! lookup {
    ($tys:ident, $ret:ident, $opt:ident) => {
        impl $tys {
            fn find_entry<'a>(
                &'a self,
                query: &str,
                directed_from: Option<&'a $ret>,
            ) -> $opt<LookupResult<&'a $ret>> {
                if let Some(entry) = self.entries.get(query) {
                    // Follow an alias if it exists
                    if let $opt::Some(also) = &entry.see_also {
                        return self.find_entry(also, Some(entry));
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
                directed_from: Option<&$ret>,
            ) -> crate::Result<Vec<LookupResult<&$ret>>>
            where
                Options: AsRef<LookupOptions> + Send + Sync,
            {
                let LookupOptions { follow, strategy } = options.as_ref();

                if let $opt::Some(entry) = self.find_entry(query, None) {
                    let result = LookupResult {
                        entry: &entry,
                        directed_from: None,
                    };

                    return Ok(vec![]);
                }

                if *strategy == LookupStrategy::Exact {
                    return Ok(vec![]);
                }

                let chars: Vec<_> = query.chars().collect();

                let mut results: Vec<LookupResult<&$ret>> = Vec::new();
                let mut start = 0;
                let mut end = chars.len();

                while start < end {
                    let substr: String = chars[start..end].iter().collect();
                    let maybe_entry = self.find_entry(substr.as_str(), None);

                    if let $opt::Some(entry) = maybe_entry {
                        let result = LookupResult {
                            entry: &entry,
                            directed_from: None,
                        };
                        results.push(entry);
                    }

                    if maybe_entry.is_some() || substr.len() <= 1 {
                        start = end;
                        end = chars.len();
                        continue;
                    }

                    end -= 1;
                }

                Ok(results)
            }

            pub fn lookup<'a, Options>(
                &'a self,
                queries: &'a Vec<&str>,
                options: Options,
            ) -> crate::Result<Vec<Vec<LookupResult<&'a $ret>>>>
            where
                Options: AsRef<LookupOptions> + Send + Sync,
            {
                queries
                    .par_iter()
                    .map(|query| self.perform_lookup(query, &options, None))
                    .collect()
            }
        }
    };
}

lookup!(Dictionary, Entry, Option);
lookup!(ArchivedDictionary, ArchivedEntry, ArchivedOption);
