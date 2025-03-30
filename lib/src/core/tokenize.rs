use charabia::{normalizer::NormalizedTokenIter, Tokenize};

use crate::{ArchivedDictionary, ArchivedEntry, Dictionary, Entry};

/* -------------------------------------------------------------------------- */
/*                                Split Options                               */
/* -------------------------------------------------------------------------- */

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

// macro_rules! tokenize {
//     ($t:ident, $r:ident) => {
//         impl $t {
//             pub fn tokenize<Options: AsRef<TokenizeOptions>>(
//                 &self,
//                 text: &str,
//                 options: Options,
//             ) -> crate::Result<Vec<&$r>> {
//                 let tokens = text.tokenize();

//                 let mut entries: Vec<&Entry> = Vec::new();

//                 for token in tokens {
//                     let split_entries = self.split(token.text(), options.as_ref())?;

//                     entries.push(Token {
//                         token: token.text().to_string(),
//                         lemma: token.lemma().to_string(),
//                         entries: split_entries,
//                     });
//                 }

//                 Ok(entries)
//             }
//         }
//     };
// }

use rayon::prelude::*;

use super::split::SplitOptions;

struct Token<T> {
    token: String,
    lemma: String,
    entries: Vec<T>,
}

unsafe impl Send for NormalizedTokenIter<'_> {}
unsafe impl Sync for NormalizedTokenIter<'_> {}

impl Dictionary {
    pub fn tokenize<Options: AsRef<TokenizeOptions>>(
        &self,
        text: &str,
        options: Options,
    ) -> crate::Result<Vec<Token<&Entry>>> {
        let tokens = text.tokenize();

        let results = tokens
            .par_bridge()
            .into_par_iter()
            .map(|token| {
                let split_entries = self.split(
                    token.text(),
                    SplitOptions::default().threshold(options.as_ref().threshold),
                )?;

                Ok(Token {
                    token: token.text().to_string(),
                    lemma: token.lemma().to_string(),
                    entries: split_entries,
                })
            })
            .collect::<crate::Result<Vec<_>>>()?
            .into_iter();

        Ok(results)
    }
}

// split!(Dictionary, Entry);
// split!(ArchivedDictionary, ArchivedEntry);
