use sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Iden, Query, Table};

use crate::{Example, ID};

use super::{definitions::Definitions, notes::Notes, utils::SQLBuilder};

#[derive(Iden)]
pub enum Examples {
    Table,
    ID,
    Value,
    Index,
    DefinitionID,
    NoteID,
}

pub fn create_examples(builder: &mut SQLBuilder) {
    builder.add_table(
        Table::create()
            .table(Examples::Table)
            .if_not_exists()
            .col(ColumnDef::new(Examples::ID).uuid().not_null().primary_key())
            .col(ColumnDef::new(Examples::Value).string())
            .col(ColumnDef::new(Examples::Index).integer().not_null())
            .col(ColumnDef::new(Examples::DefinitionID).uuid())
            .col(ColumnDef::new(Examples::NoteID).uuid())
            .foreign_key(
                ForeignKey::create()
                    .from(Examples::Table, Examples::DefinitionID)
                    .to(Definitions::Table, Definitions::ID)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Examples::Table, Examples::DefinitionID)
                    .to(Notes::Table, Notes::ID)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            ),
    )
}

pub fn insert_example(
    builder: &mut SQLBuilder,
    definition_id: Option<&str>,
    note_id: Option<&str>,
    index: u32,
    example: &Example,
) -> crate::Result<()> {
    builder.add_insert(
        Query::insert()
            .into_table(Examples::Table)
            .columns([
                Examples::ID,
                Examples::Value,
                Examples::Index,
                Examples::DefinitionID,
                Examples::NoteID,
            ])
            .values([
                ID::default().as_str().into(),
                example.value.as_str().into(),
                index.into(),
                definition_id.map_or(Expr::cust("NULL"), |id| id.into()),
                note_id.map_or(Expr::cust("NULL"), |id| id.into()),
            ])?,
    );

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_null() {
        let builder = &mut SQLBuilder::new(crate::format::sql::SQLDialect::Postgres);

        let example = Example {
            pronunciations: vec![],
            value: String::from("test"),
            translations: vec![],
        };

        let result = insert_example(builder, None, None, 0, &example);

        println!("{}", builder.build().unwrap());
        assert!(result.is_ok());
        assert!(builder.build().unwrap().ends_with("'test', 0, NULL, NULL)"));
    }
}
