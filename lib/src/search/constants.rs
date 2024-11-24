use std::sync::LazyLock;

use tantivy::tokenizer::TextAnalyzer;

#[cfg(feature = "charabia")]
use super::charabia::CharabiaTokenizer;

#[cfg(not(feature = "charabia"))]
use tantivy::tokenizer::{LowerCaser, RemoveLongFilter, SimpleTokenizer};

pub const CUSTOM_TOKENIZER: &str = "CUSTOM_TOKENIZER";

pub const DEFAULT_TOKENIZER: LazyLock<TextAnalyzer> = LazyLock::new(|| {
    #[cfg(not(feature = "charabia"))]
    return TextAnalyzer::builder(SimpleTokenizer::default())
        .filter(RemoveLongFilter::limit(40))
        .filter(LowerCaser)
        .build();

    #[cfg(feature = "charabia")]
    return CharabiaTokenizer::default().into();
});
