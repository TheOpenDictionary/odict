use clap::{arg, command, Args};

use crate::CLIContext;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct LexiconArgs {
    #[arg(required = true, help = "Path to a compiled dictionary")]
    dictionary: String,
}

pub async fn lexicon<'a>(ctx: &mut CLIContext<'a>, args: &LexiconArgs) -> anyhow::Result<()> {
    let dict = ctx
        .loader
        .load(&args.dictionary)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to load dictionary: {}", e))?
        .to_dictionary()?;

    let lexicon = dict.lexicon();

    for word in lexicon {
        ctx.println(word.to_string());
    }

    Ok(())
}
