use once_cell::sync::Lazy;
use tantivy::schema::{Field, Schema, TextFieldIndexing, TextOptions, STORED, TEXT};

pub(super) const SCHEMA: Lazy<Schema> = Lazy::new(|| {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("term", TEXT | STORED);
    schema_builder.add_text_field("definitions", TEXT);
    schema_builder.add_bytes_field("buffer", STORED);
    schema_builder.build()
});

pub(super) const FIELD_TERM: Lazy<Field> = Lazy::new(|| SCHEMA.get_field("term").unwrap());

pub(super) const FIELD_DEFINITIONS: Lazy<Field> =
    Lazy::new(|| SCHEMA.get_field("definitions").unwrap());

pub(super) const FIELD_BUFFER: Lazy<Field> = Lazy::new(|| SCHEMA.get_field("buffer").unwrap());
