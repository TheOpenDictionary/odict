use clap::{arg, Args};
use odict::alias::AliasManager;

use crate::load_dictionary;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct SetArgs {
    #[arg(required = true, help = "Name of the alias")]
    name: String,

    #[arg(required = true, help = "Dictionary path")]
    path: String,
}

pub async fn set<'a>(args: &SetArgs, overwrite: bool) -> anyhow::Result<()> {
    let dict = internal::load_dictionary(args.path.as_str()).await?;

    if overwrite {
        anyhow::Ok(AliasManager::default().set(args.name.as_str(), &dict)?)
    } else {
        anyhow::Ok(AliasManager::default().add(args.name.as_str(), &dict)?)
    }
}
