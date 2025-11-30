use clap::{arg, Args};
use odict::{alias::AliasManager, download::DictionaryDownloader, LoadOptions, OpenDictionary};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct SetArgs {
    #[arg(required = true, help = "Name of the alias")]
    name: String,

    #[arg(required = true, help = "Dictionary path")]
    path: String,

    #[arg(
        short = 'r',
        long,
        default_value_t = crate::DEFAULT_RETRIES,
        help = "Number of times to retry loading the dictionary (remote-only)"
    )]
    retries: u32,
}

pub async fn set<'a>(args: &SetArgs, overwrite: bool) -> anyhow::Result<()> {
    let dict = OpenDictionary::load_with_options(
        args.path.as_str(),
        LoadOptions::default()
            .with_downloader(DictionaryDownloader::default().with_retries(args.retries)),
    )
    .await?;

    if overwrite {
        anyhow::Ok(AliasManager::default().set(args.name.as_str(), &dict)?)
    } else {
        anyhow::Ok(AliasManager::default().add(args.name.as_str(), &dict)?)
    }
}
