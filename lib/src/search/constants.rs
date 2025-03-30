use std::sync::LazyLock;

use tantivy::tokenizer::TextAnalyzer;

use super::charabia::CharabiaTokenizer;

pub const CUSTOM_TOKENIZER: &str = "CUSTOM_TOKENIZER";

pub const DEFAULT_TOKENIZER: LazyLock<TextAnalyzer> = LazyLock::new(|| {
    return CharabiaTokenizer::default().into();
});
