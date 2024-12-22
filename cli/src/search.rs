
use clap::{arg, command, Args};
use odict::search::{get_default_index_dir, SearchOptions};

use crate::{enums::PrintFormat, print_entries, CLIContext, IndexArgs, DEFAULT_INDEX_MEMORY};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct SearchArgs {
    /// Path to a compiled dictionary or an alias
    #[arg(required = true)]
    dictionary: String,

    /// Format in which to print the results
    #[arg(short, long, default_value_t = PrintFormat::JSON)]
    format: PrintFormat,

    /// Creates a new index if one doesn't already exist
    #[arg(long, default_value_t = false)]
    index: bool,

    /// Search query
    #[arg(required = true)]
    query: String,
}

pub fn search(ctx: &mut CLIContext, args: &SearchArgs) -> anyhow::Result<()> {
    let file = ctx
        .reader
        .read_from_path_or_alias_with_manager(&args.dictionary, &ctx.alias_manager)?;

    let dict = file.to_dictionary()?;

    if args.index {
        let index_path = get_default_index_dir().join(dict.id.as_str());

        if !index_path.exists() {
            crate::index(
                ctx,
                &IndexArgs {
                    dictionary: args.dictionary.clone(),
                    directory: None,
                    overwrite: false,
                    memory: DEFAULT_INDEX_MEMORY,
                },
            )?;
        }
    }

    let results = dict.search(args.query.as_str(), SearchOptions::default())?;

    print_entries(ctx, vec![results], &args.format)?;

    Ok(())
}
