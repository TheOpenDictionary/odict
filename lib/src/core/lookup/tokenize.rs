use charabia::Segment;
use rayon::prelude::*;

use super::{
    options::{LookupOptions, LookupStrategy},
    LookupResult,
};

use crate::{ArchivedDictionary, ArchivedEntry, Dictionary, Entry};

pub type Language = charabia::Language;
pub type TokenKind = charabia::TokenKind;
pub type Script = charabia::Script;

/* ----------------------------------------------------------------------------- */
/*                                Tokenize Options                               */
/* ----------------------------------------------------------------------------- */

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<T> {
    pub lemma: String,
    pub start: usize,
    pub end: usize,
    pub kind: TokenKind,
    pub script: Script,
    pub language: Option<Language>,
    pub entries: Vec<LookupResult<T>>,
}

pub struct TokenizeOptions {
    pub follow: bool,
    // The list of languages to be considered during tokenization. Defaults to all languages supported by whatlang.
    pub allow_list: Option<Vec<Language>>,
}

impl AsRef<TokenizeOptions> for TokenizeOptions {
    fn as_ref(&self) -> &TokenizeOptions {
        self
    }
}

impl TokenizeOptions {
    pub fn default() -> Self {
        Self {
            allow_list: None,
            follow: true,
        }
    }

    pub fn follow(mut self, follow: bool) -> Self {
        self.follow = follow;
        self
    }

    pub fn allow_list(mut self, allow_list: Vec<Language>) -> Self {
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
            pub fn tokenize<'a, 'b, Options>(
                &'a self,
                text: &'b str,
                options: Options,
            ) -> crate::Result<Vec<Token<&'a $r>>>
            where
                Options: AsRef<TokenizeOptions> + Send + Sync,
            {
                let opts = options.as_ref();

                let results = text
                    .segment_with_option(
                        None,
                        options.as_ref().allow_list.as_ref().map(|v| v.as_slice()),
                    )
                    .filter(|token| !token.is_separator() && is_valid_token(token.lemma()))
                    .collect::<Vec<_>>()
                    .par_iter()
                    .map(|token| {
                        let lemma = token.lemma();

                        let script = token.script;

                        let lookup_method = if script == Script::Cj {
                            LookupStrategy::Split(1)
                        } else {
                            LookupStrategy::Exact
                        };

                        let query = vec![lemma];

                        let entries = self.lookup(
                            &query,
                            LookupOptions::default()
                                .strategy(lookup_method)
                                .follow(opts.follow),
                        )?;

                        Ok(Token {
                            lemma: lemma.to_string(),
                            start: token.char_start,
                            end: token.char_end,
                            kind: token.kind,
                            script,
                            language: token.language,
                            entries,
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
