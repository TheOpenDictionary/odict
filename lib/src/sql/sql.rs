use std::error::Error;

use crate::{sql::SQLDialect, Dictionary};

use super::{
    definitions::create_definitions,
    dictionaries::{create_dictionaries, insert_dictionary},
    entries::create_entries,
    etymologies::create_etymologies,
    examples::create_examples,
    groups::create_groups,
    notes::create_notes,
    senses::create_senses,
    utils::SQLBuilder,
};

pub trait ToSQL {
    fn to_sql(self, dialect: SQLDialect) -> Result<String, Box<dyn Error>>;
}

fn add_schema(builder: &mut SQLBuilder) {
    create_dictionaries(builder);
    create_entries(builder);
    create_etymologies(builder);
    create_senses(builder);
    create_groups(builder);
    create_definitions(builder);
    create_notes(builder);
    create_examples(builder);
}

impl ToSQL for Dictionary {
    fn to_sql(self, dialect: SQLDialect) -> Result<String, Box<dyn Error>> {
        let mut builder = SQLBuilder::new(dialect);

        add_schema(&mut builder);

        insert_dictionary(&mut builder, &self)?;

        builder.build()
    }
}
