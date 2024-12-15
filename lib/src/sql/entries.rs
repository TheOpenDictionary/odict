use std::error::Error;

use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, Iden, Query, Table};

use crate::{Entry, ID};

use super::{dictionaries::Dictionaries, etymologies::insert_etymology, utils::SQLBuilder};

#[derive(Iden)]
pub enum Entries {
    Table,
    ID,
    Term,
    DictionaryID,
}

pub fn create_entries(builder: &mut SQLBuilder) {
    builder.add_table(
        Table::create()
            .table(Entries::Table)
            .if_not_exists()
            .col(ColumnDef::new(Entries::ID).uuid().not_null().primary_key())
            .col(ColumnDef::new(Entries::Term).string().not_null())
            .col(ColumnDef::new(Entries::DictionaryID).uuid().not_null())
            .foreign_key(
                ForeignKey::create()
                    .from(Entries::Table, Entries::DictionaryID)
                    .to(Dictionaries::Table, Dictionaries::ID)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            ),
    )
}

pub fn insert_entry(
    builder: &mut SQLBuilder,
    dictionary_id: &str,
    entry: &Entry,
) -> crate::Result<()> {
    let id = ID::default();

    builder.add_insert(
        Query::insert()
            .into_table(Entries::Table)
            .columns([Entries::ID, Entries::Term, Entries::DictionaryID])
            .values([
                id.as_str().into(),
                entry.term.as_str().into(),
                dictionary_id.into(),
            ])?,
    );

    for ety in &entry.etymologies {
        insert_etymology(builder, id.as_str(), ety)?;
    }

    Ok(())
}
