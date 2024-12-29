use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, Iden, Query, Table};

use crate::{Group, ID};

use super::{senses::Senses, utils::SQLBuilder};

#[derive(Iden)]
pub enum Groups {
    Table,
    ID,
    Description,
    Index,
    SenseID,
}

pub fn create_groups(builder: &mut SQLBuilder) {
    builder.add_table(
        Table::create()
            .table(Groups::Table)
            .if_not_exists()
            .col(ColumnDef::new(Groups::ID).uuid().not_null().primary_key())
            .col(ColumnDef::new(Groups::Description).string())
            .col(ColumnDef::new(Groups::Index).integer().not_null())
            .col(ColumnDef::new(Groups::SenseID).uuid().not_null())
            .foreign_key(
                ForeignKey::create()
                    .from(Groups::Table, Groups::SenseID)
                    .to(Senses::Table, Senses::ID)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            ),
    )
}

pub fn insert_group(
    builder: &mut SQLBuilder,
    sense_id: &str,
    index: u32,
    group: &Group,
) -> crate::Result<()> {
    let id = ID::default();

    builder.add_insert(
        Query::insert()
            .into_table(Groups::Table)
            .columns([
                Groups::ID,
                Groups::Description,
                Groups::Index,
                Groups::SenseID,
            ])
            .values([
                id.as_str().into(),
                group.description.as_str().into(),
                index.into(),
                sense_id.into(),
            ])?,
    );

    Ok(())
}
