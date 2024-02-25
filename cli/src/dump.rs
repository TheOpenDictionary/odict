use std::{error::Error, fs};

use clap::{arg, command, Args};
use odict::dump::ToXML;

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

pub fn dump(ctx: &mut CLIContext, args: &DumpArgs) -> Result<(), Box<dyn Error>> {
    let DumpArgs {
        input,
        format,
        output,
    } = args;

    let dict = ctx
        .reader
        .read_from_path_or_alias_with_manager(input, &ctx.alias_manager)?
        .to_dictionary()?;

    let contents = match format {
        DumpFormat::XML => dict.to_xml(true)?,
        DumpFormat::SQLite => todo!(),
        DumpFormat::Postgres => todo!(),
        DumpFormat::MySQL => todo!(),
    };

    match output {
        Some(out) => fs::write(&out, &contents)?,
        None => ctx.println(contents),
    };

    Ok(())
}
