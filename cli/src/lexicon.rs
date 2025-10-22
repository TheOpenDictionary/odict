use clap::{arg, command, Args};
use odict::OpenDictionary;

use crate::CLIContext;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct LexiconArgs {
    #[arg(required = true, help = "Path to a compiled dictionary")]
    dictionary: String,
}

pub async fn lexicon<'a>(ctx: &mut CLIContext<'a>, args: &LexiconArgs) -> anyhow::Result<()> {
    let file = OpenDictionary::load(&args.dictionary).await?;

    let dict = file.contents()?;
    let lexicon = dict.lexicon();

    for word in lexicon {
        ctx.println(word);
    }

    Ok(())
}
