use either::Either;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct TokenizeOptions {
    #[pyo3(get, set)]
    pub follow: Option<Either<bool, u32>>,

    #[pyo3(get, set)]
    pub insensitive: Option<bool>,
}

#[pymethods]
impl TokenizeOptions {
    #[new]
    #[pyo3(signature = (follow=None, insensitive=None))]
    pub fn new(follow: Option<Either<bool, u32>>, insensitive: Option<bool>) -> Self {
        TokenizeOptions {
            follow,
            insensitive,
        }
    }
}

impl Default for TokenizeOptions {
    fn default() -> Self {
        TokenizeOptions {
            follow: None,
            insensitive: None,
        }
    }
}

impl From<TokenizeOptions> for odict::tokenize::TokenizeOptions {
    fn from(opts: TokenizeOptions) -> Self {
        let mut options = odict::tokenize::TokenizeOptions::default();

        if let Some(follow) = opts.follow {
            options = options.follow(match follow {
                Either::Left(bool_val) => bool_val,
                Either::Right(0) => false,
                Either::Right(_) => true, // Any non-zero number means follow
            });
        }

        if let Some(insensitive) = opts.insensitive {
            options = options.insensitive(insensitive);
        }

        options
    }
}
