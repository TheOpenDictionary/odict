use std::error::Error;

use crate::{Dictionary, SQLDialect};

use super::{
    dictionaries::{create_dictionaries, insert_dictionary},
    utils::SQLBuilder,
};

pub trait ToSQL {
    fn to_sql(self, dialect: SQLDialect) -> Result<String, Box<dyn Error>>;
}

fn add_schema(builder: &mut SQLBuilder) {
    create_dictionaries(builder);
}

impl ToSQL for Dictionary {
    fn to_sql(self, dialect: SQLDialect) -> Result<String, Box<dyn Error>> {
        let mut builder = SQLBuilder::new(dialect);

        add_schema(&mut builder);

        insert_dictionary(&mut builder, self);

        builder.build()
    }
}
