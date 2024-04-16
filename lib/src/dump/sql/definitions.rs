use std::error::Error;

use sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Iden, Query, Table};

use crate::{Definition, MarkdownStrategy, ID};

use super::{
    examples::insert_example, groups::Groups, notes::insert_note, senses::Senses, utils::SQLBuilder,
};

#[derive(Iden)]
pub enum Definitions {
    Table,
    ID,
    Value,
    Index,
    SenseID,
    GroupID,
}

pub fn create_definitions(builder: &mut SQLBuilder) {
    builder.add_table(
        Table::create()
            .table(Definitions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Definitions::ID)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Definitions::Value).string().not_null())
            .col(ColumnDef::new(Definitions::Index).integer().not_null())
            .col(ColumnDef::new(Definitions::SenseID).uuid())
            .col(ColumnDef::new(Definitions::GroupID).uuid())
            .foreign_key(
                ForeignKey::create()
                    .from(Definitions::Table, Definitions::SenseID)
                    .to(Senses::Table, Senses::ID)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Definitions::Table, Definitions::GroupID)
                    .to(Groups::Table, Groups::ID)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            ),
    )
}

pub fn insert_definition(
    builder: &mut SQLBuilder,
    sense_id: Option<&str>,
    group_id: Option<&str>,
    index: u32,
    definition: &Definition,
) -> Result<(), Box<dyn Error>> {
    let id = ID::default();

    builder.add_insert(
        Query::insert()
            .into_table(Definitions::Table)
            .columns([
                Definitions::ID,
                Definitions::Value,
                Definitions::Index,
                Definitions::SenseID,
                Definitions::GroupID,
            ])
            .values([
                id.as_str().into(),
                definition.value.parse(MarkdownStrategy::Disabled).into(),
                index.into(),
                sense_id.map_or(Expr::cust("NULL"), |s| s.into()),
                group_id.map_or(Expr::cust("NULL"), |g| g.into()),
            ])?,
    );

    for (i, note) in definition.notes.iter().enumerate() {
        insert_note(builder, id.as_str(), i as u32, note)?;
    }

    for (i, example) in definition.examples.iter().enumerate() {
        insert_example(builder, Some(id.as_str()), None, i as u32, example)?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::MDString;

    use super::*;

    #[test]
    fn test_null() {
        let builder = &mut SQLBuilder::new(crate::SQLDialect::Postgres);

        let definition = Definition {
            id: None,
            examples: vec![],
            notes: vec![],
            value: MDString::from("test"),
        };

        let result = insert_definition(builder, None, None, 0, &definition);

        assert!(result.is_ok());
        assert!(builder.build().unwrap().ends_with("'test', 0, NULL, NULL)"));
    }
}
