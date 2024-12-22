
use clap::{arg, Args};

use crate::CLIContext;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct SetArgs {
    #[arg(required = true, help = "Name of the alias")]
    name: String,

    #[arg(required = true, help = "Dictionary path")]
    path: String,
}

pub fn set(ctx: &mut CLIContext, args: &SetArgs, overwrite: bool) -> anyhow::Result<()> {
    let dict = ctx.reader.read_from_path(args.path.as_str())?;

    if overwrite {
        anyhow::Ok(ctx.alias_manager.set(args.name.as_str(), &dict)?)
    } else {
        anyhow::Ok(ctx.alias_manager.add(args.name.as_str(), &dict)?)
    }
}
