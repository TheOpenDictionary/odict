use std::sync::LazyLock;

use crate::{
    err::Error, split::SplitOptions, ArchivedDictionary, ArchivedEntry, Dictionary, Entry,
};

use rayon::prelude::*;
use std::marker::{Send, Sync};

use regex::Regex;

/* -------------------------------------------------------------------------- */
/*                               Structs & Enums                              */
/* -------------------------------------------------------------------------- */

/* ----------------------------- Lookup Options ----------------------------- */

#[derive(Debug, Clone)]
pub struct LookupOptions {
    pub follow: bool,
    pub split: usize,
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
            split: 0,
        }
    }

    pub fn follow(mut self, follow: bool) -> Self {
        self.follow = follow;
        self
    }

    pub fn split(mut self, threshold: usize) -> Self {
        self.split = threshold;
        self
    }
}

/* --------------------------------- Queries -------------------------------- */

#[derive(Debug, Clone)]
pub struct LookupQuery {
    pub term: String,
    pub fallback: String,
}

impl LookupQuery {
    pub fn new(term: &str) -> Self {
        Self {
            term: term.to_string(),
            fallback: term.to_string(),
        }
    }

    pub fn with_fallback(mut self, fallback: &str) -> Self {
        self.fallback = fallback.to_string();
        self
    }
}

impl AsRef<LookupQuery> for LookupQuery {
    fn as_ref(&self) -> &Self {
        self
    }
}

const PARENTHETICAL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\((.+)\)$").unwrap());

impl From<&LookupQuery> for LookupQuery {
    fn from(query: &LookupQuery) -> Self {
        query.to_owned()
    }
}

impl<S: AsRef<str>> From<S> for LookupQuery {
    fn from(query: S) -> Self {
        let term: String;
        let q = query.as_ref();

        let fallback = match PARENTHETICAL_REGEX.captures(q) {
            Some(caps) => {
                let fallback = &caps[1];
                term = q.replace(&caps[0], "");
                fallback.to_string()
            }
            None => {
                term = q.to_string();
                "".to_string()
            }
        };

        LookupQuery { term, fallback }
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Methods                                  */
/* -------------------------------------------------------------------------- */

macro_rules! lookup {
    ($tys:ident, $ret:ident) => {
        impl $tys {
            fn lookup_<Options: AsRef<LookupOptions> + Send + Sync>(
                &self,
                query: &LookupQuery,
                options: Options,
            ) -> crate::Result<Vec<&$ret>> {
                let mut entries: Vec<&$ret> = Vec::new();

                let LookupQuery { term, fallback } = query;
                let LookupOptions { follow, split } = options.as_ref();

                let mut found = self.entries.get(term.as_str());

                if found.is_none() && !fallback.is_empty() {
                    found = self.entries.get(fallback.as_str());
                }

                if let Some(entry) = found {
                    if *follow {
                        let also = &entry.see_also;

                        if also.is_some() {
                            return self.lookup_(
                                &LookupQuery {
                                    term: also.as_ref().unwrap().to_string(),
                                    fallback: fallback.to_string(),
                                },
                                options,
                            );
                        } else {
                            entries.push(entry);
                        }
                    } else {
                        entries.push(entry);
                    }
                } else if *split > 0 {
                    let split = self.split(term, &SplitOptions::default().threshold(*split))?;
                    entries.extend_from_slice(&split);
                }

                Ok(entries)
            }

            pub fn lookup<'a, Query, Options>(
                &self,
                queries: &'a Vec<Query>,
                options: Options,
            ) -> crate::Result<Vec<Vec<&$ret>>>
            where
                Query: Into<LookupQuery> + Send + Sync,
                Options: AsRef<LookupOptions> + Send + Sync,
                LookupQuery: From<&'a Query>,
            {
                queries
                    .par_iter()
                    .map(|query| self.lookup_(&query.into(), &options))
                    .collect()
            }
        }
    };
}

lookup!(Dictionary, Entry);
lookup!(ArchivedDictionary, ArchivedEntry);
