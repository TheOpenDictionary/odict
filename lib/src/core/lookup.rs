use std::error::Error;

use crate::{ArchivedDictionary, ArchivedEntry, Dictionary, Entry, SplitOptions};

use once_cell::sync::Lazy;
use rayon::prelude::*;

use regex::Regex;
use rkyv::Archived;

/* -------------------------------------------------------------------------- */
/*                               Structs & Enums                              */
/* -------------------------------------------------------------------------- */

/* ----------------------------- Lookup Options ----------------------------- */

pub struct LookupOptions {
    follow: bool,
    split: usize,
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

struct LookupQuery {
    term: String,
    fallback: String,
}

const PARENTHETICAL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\((.+)\)$").unwrap());

fn parse_query(query: &str) -> LookupQuery {
    let term: String;

    let fallback = match PARENTHETICAL_REGEX.captures(&query) {
        Some(caps) => {
            let fallback = &caps[1];
            term = query.replace(&caps[0], "");
            fallback.to_string()
        }
        None => {
            term = query.to_string();
            "".to_string()
        }
    };

    LookupQuery { term, fallback }
}

/* -------------------------------------------------------------------------- */
/*                                   Methods                                  */
/* -------------------------------------------------------------------------- */

macro_rules! lookup {
    ($tys:ident, $ret:ident) => {
        impl $tys {
            fn lookup_(
                &self,
                query: &LookupQuery,
                options: &LookupOptions,
            ) -> Result<Vec<&$ret>, Box<dyn Error + Send>> {
                let mut entries: Vec<&$ret> = Vec::new();

                let LookupQuery { term, fallback } = query;
                let LookupOptions { follow, split } = options;

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
                        }
                    } else if *split > 0 {
                        let split = self.split(term, &SplitOptions::default().threshold(*split))?;
                        entries.extend_from_slice(&split);
                    } else {
                        entries.push(entry);
                    }
                }

                Ok(entries)
            }

            pub fn lookup(
                &self,
                queries: &Vec<String>,
                options: &LookupOptions,
            ) -> Result<Vec<Vec<&$ret>>, Box<dyn Error + Send>> {
                queries
                    .par_iter()
                    .map(|query| self.lookup_(&parse_query(query), options))
                    .collect()
            }
        }
    };
}

lookup!(Dictionary, Entry);
lookup!(ArchivedDictionary, ArchivedEntry);
