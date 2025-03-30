use charabia::Tokenize;
use rayon::prelude::*;

use crate::{split::SplitOptions, ArchivedDictionary, ArchivedEntry, Dictionary, Entry};

/* ----------------------------------------------------------------------------- */
/*                                Tokenize Options                               */
/* ----------------------------------------------------------------------------- */

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
                let tokens = text
                    .tokenize()
                    .map(|token| token.lemma().to_string())
                    .collect::<Vec<String>>();

                let results = tokens
                    .par_iter()
                    .map(|token| {
                        let split_entries = self.split(
                            token,
                            SplitOptions::default().threshold(options.as_ref().threshold),
                        )?;

                        Ok(Token {
                            lemma: token.to_string(),
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
