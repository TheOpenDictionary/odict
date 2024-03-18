#![forbid(unsafe_code)]

use charabia::Tokenize;
use tantivy_tokenizer_api::{Token, TokenStream, Tokenizer};

#[derive(Clone)]
pub struct CharabiaTokenizer;

impl Default for CharabiaTokenizer {
    fn default() -> Self {
        CharabiaTokenizer
    }
}

pub struct CharabiaTokenStream {
    tokens: Vec<Token>,
    index: usize,
}

impl TokenStream for CharabiaTokenStream {
    fn advance(&mut self) -> bool {
        if self.index < self.tokens.len() {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn token(&self) -> &Token {
        &self.tokens[self.index - 1]
    }

    fn token_mut(&mut self) -> &mut Token {
        &mut self.tokens[self.index - 1]
    }
}

impl Tokenizer for CharabiaTokenizer {
    type TokenStream<'a> = CharabiaTokenStream;

    fn token_stream(&mut self, text: &str) -> CharabiaTokenStream {
        let orig_tokens = text.tokenize();
        let mut tokens = Vec::new();

        for token in orig_tokens {
            tokens.push(Token {
                offset_from: token.byte_start,
                offset_to: token.byte_end,
                position: token.char_start,
                text: String::from(&text[token.byte_start..token.byte_end]),
                position_length: token.char_end - token.char_start,
            });
        }

        CharabiaTokenStream { tokens, index: 0 }
    }
}

#[cfg(test)]
mod tests {
    use crate::search::tokenizer::CharabiaTokenizer;
    use tantivy::tokenizer::*;

    #[test]
    fn it_works() {
        let mut tokenizer = CharabiaTokenizer {};

        let mut token_stream = tokenizer.token_stream(
            "张华考上了北京大学；李萍进了中等技术学校；我在百货公司当售货员：我们都有光明的前途",
        );

        let mut tokens = Vec::new();
        let mut token_text = Vec::new();

        while let Some(token) = token_stream.next() {
            tokens.push(token.clone());
            token_text.push(token.text.clone());
        }

        // offset should be byte-indexed
        assert_eq!(tokens[0].offset_from, 0);
        assert_eq!(tokens[0].offset_to, "张".bytes().len());
        assert_eq!(tokens[1].offset_from, "张".bytes().len());

        // check tokenized text
        assert_eq!(
            token_text,
            vec![
                "张",
                "华",
                "考上",
                "了",
                "北京大学",
                "；",
                "李",
                "萍",
                "进",
                "了",
                "中等",
                "技术学校",
                "；",
                "我",
                "在",
                "百货公司",
                "当",
                "售货员",
                "：",
                "我们",
                "都",
                "有",
                "光明",
                "的",
                "前途"
            ]
        );
    }
}
