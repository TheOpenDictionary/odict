use sea_query::{ColumnDef, Iden, Query, Table};

use crate::schema::{Dictionary, Entry};

use super::{entries::insert_entry, utils::SQLBuilder};

#[derive(Iden)]
pub enum Dictionaries {
    Table,
    ID,
    Name,
}

pub fn create_dictionaries(builder: &mut SQLBuilder) {
    builder.add_table(
        Table::create()
            .table(Dictionaries::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Dictionaries::ID)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Dictionaries::Name).string()),
    )
}

pub fn insert_dictionary(builder: &mut SQLBuilder, dictionary: &Dictionary) -> crate::Result<()> {
    builder.add_insert(
        Query::insert()
            .into_table(Dictionaries::Table)
            .columns([Dictionaries::ID, Dictionaries::Name])
            .values([
                dictionary.id.as_str().into(),
                dictionary.name.as_ref().unwrap_or(&String::new()).into(),
            ])?,
    );

    let entries = dictionary.entries.iter().collect::<Vec<&Entry>>();

    for entry in entries {
        insert_entry(builder, dictionary.id.as_str(), entry)?;
    }

    Ok(())
}
