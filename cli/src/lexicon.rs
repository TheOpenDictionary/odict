use std::error::Error;

use clap::{arg, command, Args};

use crate::CLIContext;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct LexiconArgs {
    #[arg(required = true, help = "Path to a compiled dictionary")]
    dictionary: String,
}

pub fn lexicon(ctx: &mut CLIContext, args: &LexiconArgs) -> Result<(), Box<dyn Error>> {
    let dict = ctx
        .reader
        .read_from_path_or_alias_with_manager(&args.dictionary, &ctx.alias_manager)?
        .to_dictionary()?;

    let lexicon = dict.lexicon();

    for word in lexicon {
        ctx.println(word.to_string());
    }

    Ok(())
}
