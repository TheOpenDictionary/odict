use std::{error::Error, path::PathBuf};

use clap::{arg, command, Args};
use odict::search::IndexOptions;

use crate::CLIContext;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct IndexArgs {
    /// Path to a compiled dictionary
    #[arg(required = true)]
    dictionary: String,

    /// Custom directory to store the index
    #[arg(short)]
    directory: Option<PathBuf>,

    /// Whether to overwrite the index if it already exists
    #[arg(short = 'f', default_value_t = false)]
    overwrite: bool,

    /// Memory arena per thread in bytes. Must be above 15MB.
    #[arg(short, default_value_t = 15000000)]
    memory: usize,
}

pub fn index(ctx: &mut CLIContext, args: &IndexArgs) -> Result<(), Box<dyn Error>> {
    let file = ctx
        .reader
        .read_from_path_or_alias_with_manager(&args.dictionary, &ctx.alias_manager)?;

    let archive = file.to_archive()?;

    let options = IndexOptions::default()
        .overwrite(args.overwrite)
        .memory(args.memory)
        .on_item(|i, _| {
            println!("{}", i);
        });

    archive.index(&options)?;

    Ok(())
}
