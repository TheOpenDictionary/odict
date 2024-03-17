// Adapted from the Cangjia tantivy tokenizer
// https://raw.githubusercontent.com/DCjanus/cang-jie/master/src/tokenizer.rs

use charabia::Tokenize;

use super::stream::CharabiaTokenStream;

#[derive(Clone, Debug)]
pub struct CharabiaTokenizer {}

impl Default for CharabiaTokenizer {
    fn default() -> Self {
        CharabiaTokenizer {}
    }
}

impl ::tantivy::tokenizer::Tokenizer for CharabiaTokenizer {
    type TokenStream<'a> = CharabiaTokenStream<'a>;

    /// Cut text into tokens
    fn token_stream<'a>(&mut self, text: &'a str) -> CharabiaTokenStream<'a> {
        let tokens: Vec<String> = text
            .tokenize()
            .map(|tok| text[tok.byte_start..tok.byte_end].to_string())
            .collect();

        CharabiaTokenStream::new(text, tokens)
    }
}

// Adapted from the Cangjia tokenizer test
// https://raw.githubusercontent.com/DCjanus/cang-jie/master/tests/position.rs
#[cfg(test)]
mod tests {

    use tantivy::{
        collector::TopDocs,
        doc,
        query::QueryParser,
        schema::{IndexRecordOption, SchemaBuilder, TextFieldIndexing, TextOptions},
        Index, SnippetGenerator,
    };

    use crate::search::constants::CHARABIA;

    use super::CharabiaTokenizer;

    #[test]
    fn test_tokenizer_position() -> tantivy::Result<()> {
        let mut schema_builder = SchemaBuilder::default();

        let text_indexing = TextFieldIndexing::default()
            .set_tokenizer(CHARABIA) // Set custom tokenizer
            .set_index_option(IndexRecordOption::WithFreqsAndPositions);

        let text_options = TextOptions::default()
            .set_indexing_options(text_indexing)
            .set_stored();

        let title = schema_builder.add_text_field("title", text_options);
        let schema = schema_builder.build();

        let index = Index::create_in_ram(schema);
        index.tokenizers().register(CHARABIA, tokenizer()); // Build cang-jie Tokenizer

        let mut index_writer = index.writer(50 * 1024 * 1024)?;

        index_writer.add_document(doc! { title => "南京大桥" })?;
        index_writer.add_document(doc! { title => "这个是长江" })?;
        index_writer.add_document(doc! { title => "这个是南京长" })?;
        index_writer.commit()?;

        let reader = index.reader()?;
        let searcher = reader.searcher();

        let query = QueryParser::for_index(&index, vec![title]).parse_query("南京")?;
        let top_docs = searcher.search(query.as_ref(), &TopDocs::with_limit(10000))?;

        let snippet = SnippetGenerator::create(&searcher, &query, title).unwrap();

        for doc in top_docs.iter() {
            let s = snippet.snippet_from_doc(&searcher.doc(doc.1).unwrap());
            dbg!(s.to_html());
        }
        Ok(())
    }

    fn tokenizer() -> CharabiaTokenizer {
        CharabiaTokenizer::default()
    }
}
