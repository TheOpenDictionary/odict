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
    /// Whether to follow see_also links until finding an entry with etymologies.
    /// true means follow redirects until etymology found, false means no following.
    pub follow: bool,
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
            follow: false,
            strategy: LookupStrategy::Exact,
            insensitive: false,
        }
    }

    pub fn follow(mut self, follow: bool) -> Self {
        self.follow = follow;
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
                follow: &bool,
                insensitive: &bool,
                query: &str,
                directed_from: Option<&'a $ret>,
                path: &mut Vec<String>,
            ) -> crate::Result<$opt<LookupResult<&'a $ret>>> {
                // Check for redirect loop
                if path.contains(&query.to_string()) {
                    // Build the loop chain from the path in order
                    let mut chain = path.clone();
                    chain.push(query.to_string());
                    return Err(crate::Error::RedirectLoop(chain.join(" -> ")));
                }

                // Add current query to path
                path.push(query.to_string());

                // Try exact match first
                if let Some(entry) = self.entries.get(query) {
                    // Follow an alias if follow is true, entry has no etymologies, and see_also exists
                    if *follow && entry.etymologies.is_empty() {
                        if let Option::Some(also) = &entry.see_also.as_ref() {
                            if also.len() > 0 {
                                // Recursively follow the redirect
                                let result = self.find_entry(
                                    follow,
                                    insensitive,
                                    also,
                                    directed_from.or(Some(entry)),
                                    path,
                                );

                                path.pop();

                                return result;
                            }
                        }
                    }

                    path.pop();

                    return Ok($opt::Some(LookupResult {
                        entry,
                        directed_from,
                    }));
                }

                // If insensitive flag is true and exact match failed, try with lowercase
                if *insensitive {
                    let query_lower = query.to_lowercase();

                    // Only try lowercase if it's different from the original query
                    if query_lower != query {
                        // Try direct lookup with lowercase (keep insensitive flag for redirect following)
                        if let Ok($opt::Some(result)) =
                            self.find_entry(follow, insensitive, &query_lower, directed_from, path)
                        {
                            path.pop();
                            return Ok($opt::Some(result));
                        }
                    }
                }

                // Remove from path since we're not following any redirects
                path.pop();

                Ok($opt::None)
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

                let mut path = Vec::new();

                return match self.find_entry(follow, insensitive, query, None, &mut path)? {
                    $opt::Some(result) => Ok(vec![result]),
                    $opt::None => {
                        let mut results: Vec<LookupResult<&$ret>> = Vec::new();

                        if let LookupStrategy::Split(min_length) = strategy {
                            let chars: Vec<_> = query.chars().collect();
                            let mut start = 0;
                            let mut end = chars.len();

                            while start < end {
                                let substr: String = chars[start..end].iter().collect();
                                let mut substr_path = Vec::new();
                                let maybe_entry = self.find_entry(
                                    follow,
                                    insensitive,
                                    substr.as_str(),
                                    None,
                                    &mut substr_path,
                                );

                                match maybe_entry {
                                    Ok($opt::Some(result)) => {
                                        results.push(result);
                                        start = end;
                                        end = chars.len();
                                        continue;
                                    }
                                    Ok($opt::None) => {
                                        if substr.len() <= *min_length {
                                            start = end;
                                            end = chars.len();
                                            continue;
                                        }
                                    }
                                    Err(e) => return Err(e),
                                }

                                end -= 1;
                            }
                        }

                        Ok(results)
                    }
                };
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
