use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, Iden, Query, Table};

use crate::schema::{Etymology, Sense, ID};

use super::{
    entries::Entries, pronunciations::insert_pronunciation, senses::insert_sense, utils::SQLBuilder,
};

#[derive(Iden)]
pub enum Etymologies {
    Table,
    ID,
    Description,
    EntryID,
}

pub fn create_etymologies(builder: &mut SQLBuilder) {
    builder.add_table(
        Table::create()
            .table(Etymologies::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Etymologies::ID)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Etymologies::Description).string())
            .col(ColumnDef::new(Etymologies::EntryID).uuid().not_null())
            .foreign_key(
                ForeignKey::create()
                    .from(Etymologies::Table, Etymologies::EntryID)
                    .to(Entries::Table, Entries::ID)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            ),
    )
}

pub fn insert_etymology(
    builder: &mut SQLBuilder,
    entry_id: &str,
    etymology: &Etymology,
) -> crate::Result<()> {
    let id = ID::default();

    builder.add_insert(
        Query::insert()
            .into_table(Etymologies::Table)
            .columns([
                Etymologies::ID,
                Etymologies::Description,
                Etymologies::EntryID,
            ])
            .values([
                id.as_str().into(),
                etymology
                    .description
                    .as_ref()
                    .unwrap_or(&String::new())
                    .into(),
                entry_id.into(),
            ])?,
    );

    // Insert pronunciations
    for pronunciation in &etymology.pronunciations {
        insert_pronunciation(builder, id.as_str(), pronunciation)?;
    }

    let senses = etymology.senses.iter().collect::<Vec<&Sense>>();

    for sense in senses {
        insert_sense(builder, id.as_str(), sense)?;
    }

    Ok(())
}
