use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, Iden, Query, Table};

use crate::{Pronunciation, ID};

use super::{etymologies::Etymologies, utils::SQLBuilder};

#[derive(Iden)]
pub enum Pronunciations {
    Table,
    ID,
    Kind,
    Value,
    EtymologyID,
}

#[derive(Iden)]
pub enum PronunciationUrls {
    Table,
    ID,
    Source,
    MimeType,
    PronunciationID,
}

pub fn create_pronunciations(builder: &mut SQLBuilder) {
    // Create the pronunciations table
    builder.add_table(
        Table::create()
            .table(Pronunciations::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Pronunciations::ID)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Pronunciations::Kind).string().not_null())
            .col(ColumnDef::new(Pronunciations::Value).string().not_null())
            .col(
                ColumnDef::new(Pronunciations::EtymologyID)
                    .uuid()
                    .not_null(),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Pronunciations::Table, Pronunciations::EtymologyID)
                    .to(Etymologies::Table, Etymologies::ID)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            ),
    );

    // Create the pronunciation_urls table
    builder.add_table(
        Table::create()
            .table(PronunciationUrls::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(PronunciationUrls::ID)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(PronunciationUrls::Source)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(PronunciationUrls::MimeType).string())
            .col(
                ColumnDef::new(PronunciationUrls::PronunciationID)
                    .uuid()
                    .not_null(),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(PronunciationUrls::Table, PronunciationUrls::PronunciationID)
                    .to(Pronunciations::Table, Pronunciations::ID)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            ),
    );
}

pub fn insert_pronunciation(
    builder: &mut SQLBuilder,
    etymology_id: &str,
    pronunciation: &Pronunciation,
) -> crate::Result<()> {
    let id = ID::default();

    // Insert the pronunciation
    builder.add_insert(
        Query::insert()
            .into_table(Pronunciations::Table)
            .columns([
                Pronunciations::ID,
                Pronunciations::Kind,
                Pronunciations::Value,
                Pronunciations::EtymologyID,
            ])
            .values([
                id.as_str().into(),
                pronunciation.kind.to_string().into(),
                pronunciation.value.clone().into(),
                etymology_id.into(),
            ])?,
    );

    // Insert any URLs
    for url in &pronunciation.media {
        let url_id = ID::default();

        builder.add_insert(
            Query::insert()
                .into_table(PronunciationUrls::Table)
                .columns([
                    PronunciationUrls::ID,
                    PronunciationUrls::Source,
                    PronunciationUrls::MimeType,
                    PronunciationUrls::PronunciationID,
                ])
                .values([
                    url_id.as_str().into(),
                    url.src.clone().into(),
                    url.mime_type.clone().unwrap_or_default().into(),
                    id.as_str().into(),
                ])?,
        );
    }

    Ok(())
}
