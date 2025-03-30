use charabia::Segment;
use rayon::prelude::*;

use crate::{split::SplitOptions, ArchivedDictionary, ArchivedEntry, Dictionary, Entry};

/* ----------------------------------------------------------------------------- */
/*                                Tokenize Options                               */
/* ----------------------------------------------------------------------------- */

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token<T> {
    pub lemma: String,
    pub entries: Vec<T>,
}

pub struct TokenizeOptions {
    threshold: usize,
}

impl AsRef<TokenizeOptions> for TokenizeOptions {
    fn as_ref(&self) -> &TokenizeOptions {
        self
    }
}

impl TokenizeOptions {
    pub fn default() -> Self {
        Self { threshold: 0 }
    }

    pub fn threshold(mut self, threshold: usize) -> Self {
        self.threshold = threshold;
        self
    }
}

/* -------------------------------------------------------------------------- */
/*                               Implementation                               */
/* -------------------------------------------------------------------------- */

macro_rules! tokenize {
    ($t:ident, $r:ident) => {
        impl $t {
            pub fn tokenize<Options>(
                &self,
                text: &str,
                options: Options,
            ) -> crate::Result<Vec<Token<&$r>>>
            where
                Options: AsRef<TokenizeOptions> + Send + Sync,
            {
                let results = text
                    .segment()
                    .filter(|token| !token.is_separator() && !token.lemma().trim().is_empty())
                    .collect::<Vec<_>>()
                    .par_iter()
                    .map(|token| {
                        let lemma = token.lemma();

                        let split_entries = self.split(
                            lemma,
                            SplitOptions::default().threshold(options.as_ref().threshold),
                        )?;

                        Ok(Token {
                            lemma: lemma.to_string(),
                            entries: split_entries,
                        })
                    })
                    .collect::<crate::Result<Vec<_>>>()?;

                Ok(results)
            }
        }
    };
}

tokenize!(Dictionary, Entry);
tokenize!(ArchivedDictionary, ArchivedEntry);
