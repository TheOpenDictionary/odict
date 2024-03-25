use once_cell::sync::Lazy;

use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, Table, TableCreateStatement, Value};

const TABLE_DICTIONARY: Lazy<TableCreateStatement> = Lazy::new(|| {
    Table::create()
        .table(Char::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(Char::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Char::FontSize).integer().not_null())
        .col(ColumnDef::new(Char::Character).string().not_null())
        .col(ColumnDef::new(Char::SizeW).integer().not_null())
        .col(ColumnDef::new(Char::SizeH).integer().not_null())
        .col(
            ColumnDef::new(Char::FontId)
                .integer()
                .default(Value::Int(None)),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_2e303c3a712662f1fc2a4d0aad6")
                .from(Char::Table, Char::FontId)
                .to(Font::Table, Font::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
});
