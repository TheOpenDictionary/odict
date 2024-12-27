use crate::{ArchivedDictionary, ArchivedEntry, Dictionary, Entry};

use nucleo_matcher::{pattern::Pattern, Config, Matcher};

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
            pub fn find<Options: AsRef<FindOptions>>(
                &self,
                pattern: &str,
                options: Options,
            ) -> Vec<&$e> {
                let lexicon: Vec<&str> = self.lexicon();
                let opts = options.as_ref();
                let mut matcher = Matcher::new(Config::DEFAULT);

                let matches = Pattern::parse(&pattern, opts.case_matching, opts.normalization)
                    .match_list(lexicon, &mut matcher);

                return self
                    .entries
                    .iter()
                    .filter(|entry| matches.iter().any(|&str| str.0 == entry.0))
                    .map(|e| e.1)
                    .collect::<Vec<&$e>>();
            }
        }
    };
}

find!(Dictionary, Entry);
find!(ArchivedDictionary, ArchivedEntry);
