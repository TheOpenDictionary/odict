use std::sync::LazyLock;

use tantivy::schema::{Field, IndexRecordOption, Schema, TextFieldIndexing, TextOptions, STORED};

use super::constants::CUSTOM_TOKENIZER;

pub(super) const SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
    let mut schema_builder = Schema::builder();

    let text_options = TextOptions::default().set_indexing_options(
        TextFieldIndexing::default()
            .set_tokenizer(CUSTOM_TOKENIZER)
            .set_index_option(IndexRecordOption::WithFreqsAndPositions),
    );

    schema_builder.add_text_field("term", text_options.clone().set_stored());
    schema_builder.add_text_field("definitions", text_options);
    schema_builder.add_bytes_field("buffer", STORED);
    schema_builder.build()
});

pub(super) const FIELD_TERM: LazyLock<Field> = LazyLock::new(|| SCHEMA.get_field("term").unwrap());

pub(super) const FIELD_DEFINITIONS: LazyLock<Field> =
    LazyLock::new(|| SCHEMA.get_field("definitions").unwrap());

pub(super) const FIELD_BUFFER: LazyLock<Field> =
    LazyLock::new(|| SCHEMA.get_field("buffer").unwrap());
