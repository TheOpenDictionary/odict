use std::error::Error;

use clap::{arg, Args};

use crate::CLIContext;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct DeleteArgs {
    #[arg(required = true, help = "Name of the alias")]
    name: String,
}

pub fn delete(ctx: &mut CLIContext, args: &DeleteArgs) -> Result<(), Box<dyn Error>> {
    ctx.alias_manager.delete(args.name.as_str())
}
