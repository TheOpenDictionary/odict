use crate::{ArchivedDictionary, ArchivedEntry, Dictionary, Entry};

use crate::fuzzy::fuzzy_find;

pub use nucleo_matcher::pattern::{CaseMatching, Normalization};

#[derive(Debug, Clone)]
pub struct FindOptions {
    pub case_matching: CaseMatching,
    pub normalization: Normalization,
}

impl FindOptions {
    pub fn case_matching(mut self, value: CaseMatching) -> Self {
        self.case_matching = value;
        self
    }

    pub fn normalization(mut self, value: Normalization) -> Self {
        self.normalization = value;
        self
    }
}

impl Default for FindOptions {
    fn default() -> Self {
        Self {
            case_matching: CaseMatching::Smart,
            normalization: Normalization::Smart,
        }
    }
}

impl AsRef<FindOptions> for FindOptions {
    fn as_ref(&self) -> &FindOptions {
        self
    }
}

macro_rules! find {
    ($t:ident, $e:ident) => {
        impl $t {
            pub fn find_terms<Options: AsRef<FindOptions>>(
                &self,
                pattern: &str,
                options: Options,
            ) -> Vec<&str> {
                let lexicon: Vec<&str> = self.lexicon();
                let opts = options.as_ref();
                let matches =
                    fuzzy_find(&lexicon, &pattern, opts.case_matching, opts.normalization);

                return matches.iter().map(|e| *e.0).collect::<Vec<&str>>();
            }

            pub fn find_entries<Options: AsRef<FindOptions>>(
                &self,
                pattern: &str,
                options: Options,
            ) -> Vec<&$e> {
                let matches = self.find_terms(pattern, options);

                return self
                    .entries
                    .iter()
                    .filter(|entry| matches.iter().any(|str| str == entry.0))
                    .map(|e| e.1)
                    .collect::<Vec<&$e>>();
            }
        }
    };
}

find!(Dictionary, Entry);
find!(ArchivedDictionary, ArchivedEntry);
