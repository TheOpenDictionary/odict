use crate::schema::{ArchivedDictionary, ArchivedEntry, Dictionary, Entry};

use rayon::prelude::*;
use rkyv::option::ArchivedOption;
use std::marker::{Send, Sync};

#[derive(Debug, PartialEq, Clone)]
pub enum LookupStrategy {
    Exact,
    Split(usize),
}

#[derive(Debug, Clone)]
pub struct LookupOptions {
    /// Maximum number of redirects to follow via see_also links.
    /// None means no following, Some(u32::MAX) provides infinite following (old behavior).
    pub follow: Option<u32>,
    pub strategy: LookupStrategy,
    pub insensitive: bool,
}

impl AsRef<LookupOptions> for LookupOptions {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl LookupOptions {
    pub fn default() -> Self {
        Self {
            follow: None,
            strategy: LookupStrategy::Exact,
            insensitive: false,
        }
    }

    pub fn follow(mut self, follow: u32) -> Self {
        self.follow = Some(follow);
        self
    }

    pub fn strategy(mut self, strategy: LookupStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn insensitive(mut self, insensitive: bool) -> Self {
        self.insensitive = insensitive;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
                follow: &Option<u32>,
                insensitive: &bool,
                query: &str,
                directed_from: Option<&'a $ret>,
            ) -> $opt<LookupResult<&'a $ret>> {
                // Try exact match first
                if let Some(entry) = self.entries.get(query) {
                    // Follow an alias if it exists and we have redirects remaining
                    if let Some(max_redirects) = follow {
                        if *max_redirects > 0 {
                            if let Option::Some(also) = &entry.see_also.as_ref() {
                                if also.len() > 0 {
                                    // Decrement redirect count for recursive call
                                    let remaining_redirects = if *max_redirects == u32::MAX {
                                        Some(u32::MAX) // Keep infinite
                                    } else {
                                        Some(max_redirects - 1)
                                    };
                                    return self.find_entry(
                                        &remaining_redirects,
                                        insensitive,
                                        also,
                                        directed_from.or(Some(entry)),
                                    );
                                }
                            }
                        }
                    }

                    return $opt::Some(LookupResult {
                        entry,
                        directed_from,
                    });
                }

                // If insensitive flag is true and exact match failed, try with lowercase
                if *insensitive {
                    let query_lower = query.to_lowercase();

                    // Only try lowercase if it's different from the original query
                    if query_lower != query {
                        // Try direct lookup with lowercase (reuse all the same logic)
                        if let $opt::Some(result) =
                            self.find_entry(follow, &false, &query_lower, directed_from)
                        {
                            return $opt::Some(result);
                        }
                    }
                }

                $opt::None
            }

            fn perform_lookup<'a, Options>(
                &'a self,
                query: &str,
                options: Options,
            ) -> crate::Result<Vec<LookupResult<&'a $ret>>>
            where
                Options: AsRef<LookupOptions> + Send + Sync,
            {
                let LookupOptions {
                    follow,
                    strategy,
                    insensitive,
                } = options.as_ref();

                if let $opt::Some(result) = self.find_entry(follow, insensitive, query, None) {
                    return Ok(vec![result]);
                }

                let mut results: Vec<LookupResult<&$ret>> = Vec::new();

                if let LookupStrategy::Split(min_length) = strategy {
                    let chars: Vec<_> = query.chars().collect();
                    let mut start = 0;
                    let mut end = chars.len();

                    while start < end {
                        let substr: String = chars[start..end].iter().collect();
                        let maybe_entry =
                            self.find_entry(follow, insensitive, substr.as_str(), None);

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

            pub fn lookup<'a, 'b, Query, Options>(
                &'a self,
                queries: &'b Vec<Query>,
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
