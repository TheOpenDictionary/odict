use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, Iden, Query, Table};

use crate::{Etymology, MarkdownStrategy, Sense, ID};

use super::{entries::Entries, senses::insert_sense, utils::SQLBuilder};

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
                    .map_or(String::new(), |d| d.parse(MarkdownStrategy::Disabled))
                    .into(),
                entry_id.into(),
            ])?,
    );

    let mut senses = etymology.senses.values().collect::<Vec<&Sense>>();

    senses.sort_by(|a, b| a.pos.cmp(&b.pos));

    for sense in senses {
        insert_sense(builder, id.as_str(), sense)?;
    }

    Ok(())
}
