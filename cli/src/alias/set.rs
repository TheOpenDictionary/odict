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

pub async fn set<'a>(
    ctx: &mut CLIContext<'a>,
    args: &SetArgs,
    overwrite: bool,
) -> anyhow::Result<()> {
    let dict = ctx
        .loader
        .load(args.path.as_str())
        .await
        .map_err(|e| anyhow::anyhow!("Failed to load dictionary: {}", e))?;

    if overwrite {
        anyhow::Ok(ctx.loader.alias_manager().set(args.name.as_str(), &dict)?)
    } else {
        anyhow::Ok(ctx.loader.alias_manager().add(args.name.as_str(), &dict)?)
    }
}
