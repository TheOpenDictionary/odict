use once_cell::sync::Lazy;
use tantivy::schema::{Schema, STORED, TEXT};

const SCHEMA: Lazy<Schema> = Lazy::new(|| {
    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field("title", TEXT | STORED);

    schema_builder.build()
});

impl Dictionary {
    pub fn index() {
        // let index_path = TempDir::new()?;
        let mut schema_builder = Schema::builder();
        let index = Index::create_in_dir(&index_path, schema.clone())?;
        schema_builder.add_text_field("title", TEXT | STORED);

        let schema = schema_builder.build();
    }
}
