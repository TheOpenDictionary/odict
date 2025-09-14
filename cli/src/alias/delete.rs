use clap::{arg, Args};
use odict::alias::AliasManager;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct DeleteArgs {
    #[arg(required = true, help = "Name of the alias")]
    name: String,
}

pub fn delete(args: &DeleteArgs) -> anyhow::Result<()> {
    anyhow::Ok(AliasManager::default().delete(args.name.as_str())?)
}
