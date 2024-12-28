use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, Iden, Query, Table};

use crate::{MarkdownStrategy, Note, ID};

use super::{definitions::Definitions, examples::insert_example, utils::SQLBuilder};

#[derive(Iden)]
pub enum Notes {
    Table,
    ID,
    Value,
    Index,
    DefinitionID,
}

pub fn create_notes(builder: &mut SQLBuilder) {
    builder.add_table(
        Table::create()
            .table(Notes::Table)
            .if_not_exists()
            .col(ColumnDef::new(Notes::ID).uuid().not_null().primary_key())
            .col(ColumnDef::new(Notes::Value).string())
            .col(ColumnDef::new(Notes::Index).integer().not_null())
            .col(ColumnDef::new(Notes::DefinitionID).uuid().not_null())
            .foreign_key(
                ForeignKey::create()
                    .from(Notes::Table, Notes::DefinitionID)
                    .to(Definitions::Table, Definitions::ID)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            ),
    )
}

pub fn insert_note(
    builder: &mut SQLBuilder,
    definition_id: &str,
    index: u32,
    note: &Note,
) -> crate::Result<()> {
    let id = ID::default();

    builder.add_insert(
        Query::insert()
            .into_table(Notes::Table)
            .columns([Notes::ID, Notes::Value, Notes::Index, Notes::DefinitionID])
            .values([
                id.as_str().into(),
                note.value.parse(MarkdownStrategy::Disabled).into(),
                index.into(),
                definition_id.into(),
            ])?,
    );

    for (i, example) in note.examples.iter().enumerate() {
        insert_example(builder, None, Some(id.as_str()), i as u32, example)?;
    }

    Ok(())
}
