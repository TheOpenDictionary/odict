use sea_query::{ColumnDef, Iden, Query, Table};

use crate::Dictionary;

use super::utils::SQLBuilder;

#[derive(Iden)]
enum Dictionaries {
    Table,
    Id,
    Name,
}

pub fn create_dictionaries(builder: &mut SQLBuilder) {
    builder.add_table(
        Table::create()
            .table(Dictionaries::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Dictionaries::Id)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Dictionaries::Name).string())
            .to_owned(),
    )
}

pub fn insert_dictionary(builder: &mut SQLBuilder, dictionary: Dictionary) {
    builder.add_insert(
        Query::insert()
            .into_table(Dictionaries::Table)
            .columns([Dictionaries::Id, Dictionaries::Name])
            .values_panic([dictionary.id.as_str().into(), dictionary.name.into()])
            .to_owned(),
    );
}
