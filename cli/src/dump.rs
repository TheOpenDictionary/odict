use std::fs;

use clap::{arg, command, Args};
use odict::{
    download::DictionaryDownloader,
    format::{
        sql::{SQLDialect, ToSQL},
        xml::ToXML,
    },
    LoadOptions, OpenDictionary,
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

    #[arg(
        short = 'r',
        long,
        default_value_t = crate::DEFAULT_RETRIES,
        help = "Number of times to retry loading the dictionary (remote-only)"
    )]
    retries: u32,
}

pub async fn dump<'a>(ctx: &mut CLIContext<'a>, args: &DumpArgs) -> anyhow::Result<()> {
    let DumpArgs {
        input,
        format,
        output,
        retries,
    } = args;

    let dict = OpenDictionary::load_with_options(
        input,
        LoadOptions::default()
            .with_downloader(DictionaryDownloader::default().with_retries(*retries)),
    )
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
