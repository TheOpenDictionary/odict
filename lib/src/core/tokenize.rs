use charabia::Segment;
use rayon::prelude::*;

use crate::{split::SplitOptions, ArchivedDictionary, ArchivedEntry, Dictionary, Entry};

pub type Language = charabia::Language;

/* ----------------------------------------------------------------------------- */
/*                                Tokenize Options                               */
/* ----------------------------------------------------------------------------- */

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token<T> {
    pub lemma: String,
    pub language: Option<String>,
    pub entries: Vec<T>,
}

pub struct TokenizeOptions<'a> {
    threshold: usize,
    allow_list: Option<&'a [Language]>,
}

impl<'a> AsRef<TokenizeOptions<'a>> for TokenizeOptions<'a> {
    fn as_ref(&self) -> &TokenizeOptions<'a> {
        self
    }
}

impl<'a> TokenizeOptions<'a> {
    pub fn default() -> Self {
        Self {
            allow_list: None,
            threshold: 0,
        }
    }

    pub fn threshold(mut self, threshold: usize) -> Self {
        self.threshold = threshold;
        self
    }

    pub fn allow_list(mut self, allow_list: &'a [Language]) -> Self {
        self.allow_list = Some(allow_list);
        self
    }
}

/* -------------------------------------------------------------------------- */
/*                               Implementation                               */
/* -------------------------------------------------------------------------- */

// Charabia isn't always reliable in determine what is whitespace/separator/etc
fn is_valid_token(input: &str) -> bool {
    let c: String = input
        .chars()
        .filter(|c| !c.is_ascii_punctuation() && !c.is_control() && !c.is_whitespace())
        .collect();

    !c.trim().is_empty()
}

macro_rules! tokenize {
    ($t:ident, $r:ident) => {
        impl $t {
            pub fn tokenize<'a, Options>(
                &self,
                text: &str,
                options: Options,
            ) -> crate::Result<Vec<Token<&$r>>>
            where
                Options: AsRef<TokenizeOptions<'a>> + Send + Sync,
            {
                let results = text
                    .segment_with_option(None, options.as_ref().allow_list)
                    .filter(|token| !token.is_separator() && is_valid_token(token.lemma()))
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
                            language: token.language.map(|lang| lang.code().to_string()),
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
