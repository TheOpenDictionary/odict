use std::error::Error;

use clap::{arg, command, Args};

use crate::CLIContext;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct MergeArgs {
    #[arg(
        required = true,
        help = "Path of the dictionary to merge into (unless --output is specified)"
    )]
    destination: String,

    #[arg(required = true, help = "Paths of dictionaries to merge")]
    sources: Vec<String>,

    #[arg(short, long, help = "Separate output path for the compiled dictionary")]
    output: Option<String>,
}

pub fn merge(ctx: &CLIContext, args: &MergeArgs) -> Result<(), Box<dyn Error>> {
    let mut dict = ctx
        .reader
        .read_from_path(&args.destination)?
        .to_dictionary()?;

    for source in &args.sources {
        let source_dict = ctx
            .reader
            .read_from_path_or_alias_with_manager(source, &ctx.alias_manager)?
            .to_dictionary()?;

        dict.merge(&source_dict);
    }

    if let Some(output) = &args.output {
        ctx.writer.write_to_path(&dict, &output)?;
    } else {
        ctx.writer.write_to_path(&dict, &args.destination)?;
    }

    Ok(())
}
