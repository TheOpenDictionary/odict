use std::fs;

use clap::{arg, command, Args};
use odict::{
    format::{
        sql::{SQLDialect, ToSQL},
        xml::ToXML,
    },
    OpenDictionary,
};

use crate::{enums::DumpFormat, CLIContext};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct DumpArgs {
    #[arg(required = true, help = "Path to a compile dictionary")]
    input: String,

    #[arg(default_value_t = DumpFormat::XML, short, help = "Format in which to dump the dictionary.")]
    format: DumpFormat,

    #[arg(short, help = "Output path of the dump. Defaults to stdout.")]
    output: Option<String>,
}

pub async fn dump<'a>(ctx: &mut CLIContext<'a>, args: &DumpArgs) -> anyhow::Result<()> {
    let DumpArgs {
        input,
        format,
        output,
    } = args;

    let dict = OpenDictionary::load(input)
        .await?
        .contents()?
        .deserialize()?;

    let contents = match format {
        DumpFormat::XML => dict.to_xml(true)?,
        DumpFormat::SQLite => dict.to_sql(SQLDialect::SQLite)?,
        DumpFormat::Postgres => dict.to_sql(SQLDialect::Postgres)?,
        DumpFormat::MySQL => dict.to_sql(SQLDialect::MySQL)?,
    };

    match output {
        Some(out) => fs::write(out, &contents)?,
        None => ctx.println(contents),
    };

    Ok(())
}
