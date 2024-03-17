// Adapted from the Cangjie Tantivy tokenizer:
// https://raw.githubusercontent.com/DCjanus/cang-jie/master/src/stream.rs

use tantivy::tokenizer::Token;

#[derive(Debug)]
pub struct CharabiaTokenStream<'a> {
    src: &'a str,
    result: Vec<String>,
    // Begin with 1
    index: usize,
    token: Token,
}

impl<'a> CharabiaTokenStream<'a> {
    pub fn new(src: &'a str, result: Vec<String>) -> Self {
        CharabiaTokenStream {
            src,
            result,
            index: 0,
            token: Token::default(),
        }
    }
}

impl<'a> ::tantivy::tokenizer::TokenStream for CharabiaTokenStream<'a> {
    fn advance(&mut self) -> bool {
        if self.index < self.result.len() {
            let current_word = &self.result[self.index];
            println!("{}", current_word);
            let offset_from = current_word.as_ptr() as usize - self.src.as_ptr() as usize;
            println!("{}", offset_from);
            let offset_to = offset_from + current_word.len();

            self.token = Token {
                offset_from,
                offset_to,
                position: self.index,
                text: current_word.to_string(),
                position_length: 1,
            };

            self.index += 1;
            true
        } else {
            false
        }
    }

    fn token(&self) -> &::tantivy::tokenizer::Token {
        &self.token
    }

    fn token_mut(&mut self) -> &mut ::tantivy::tokenizer::Token {
        &mut self.token
    }
}
