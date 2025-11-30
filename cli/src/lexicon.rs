use clap::{arg, command, Args};
use odict::{download::DictionaryDownloader, LoadOptions, OpenDictionary};

use crate::CLIContext;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct LexiconArgs {
    #[arg(required = true, help = "Path to a compiled dictionary")]
    dictionary: String,

    #[arg(
        short = 'r',
        long,
        default_value_t = crate::DEFAULT_RETRIES,
        help = "Number of times to retry loading the dictionary (remote-only)"
    )]
    retries: u32,
}

pub async fn lexicon<'a>(ctx: &mut CLIContext<'a>, args: &LexiconArgs) -> anyhow::Result<()> {
    let file = OpenDictionary::load_with_options(
        &args.dictionary,
        LoadOptions::default()
            .with_downloader(DictionaryDownloader::default().with_retries(args.retries)),
    )
    .await?;

    let dict = file.contents()?;
    let lexicon = dict.lexicon();

    for word in lexicon {
        ctx.println(word);
    }

    Ok(())
}
