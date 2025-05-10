use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, Iden, Query, Table};

use crate::{DefinitionType, Sense, ID};

use super::{
    definitions::insert_definition, etymologies::Etymologies, groups::insert_group,
    utils::SQLBuilder,
};

#[derive(Iden)]
pub enum Senses {
    Table,
    ID,
    POS,
    EtymologyID,
}

pub fn create_senses(builder: &mut SQLBuilder) {
    builder.add_table(
        Table::create()
            .table(Senses::Table)
            .if_not_exists()
            .col(ColumnDef::new(Senses::ID).uuid().not_null().primary_key())
            .col(ColumnDef::new(Senses::POS).string())
            .col(ColumnDef::new(Senses::EtymologyID).uuid().not_null())
            .foreign_key(
                ForeignKey::create()
                    .from(Senses::Table, Senses::EtymologyID)
                    .to(Etymologies::Table, Etymologies::ID)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            ),
    )
}

pub fn insert_sense(
    builder: &mut SQLBuilder,
    etymology_id: &str,
    sense: &Sense,
) -> crate::Result<()> {
    let id = ID::default();

    builder.add_insert(
        Query::insert()
            .into_table(Senses::Table)
            .columns([Senses::ID, Senses::POS, Senses::EtymologyID])
            .values([
                id.as_str().into(),
                sense.pos.to_string().into(),
                etymology_id.into(),
            ])?,
    );

    for (i, dt) in sense.definitions.iter().enumerate() {
        match dt {
            DefinitionType::Group(g) => {
                insert_group(builder, id.as_str(), i as u32, g)?;
            }
            DefinitionType::Definition(d) => {
                insert_definition(builder, Some(id.as_str()), None, i as u32, d)?;
            }
        }
    }

    Ok(())
}
