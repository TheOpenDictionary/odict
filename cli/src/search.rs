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

pub async fn search<'a>(ctx: &mut CLIContext<'a>, args: &SearchArgs) -> anyhow::Result<()> {
    let file = ctx
        .loader
        .load(&args.dictionary)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to load dictionary: {}", e))?;

    let dict = file.to_dictionary()?;

    if args.index {
        let index_path = get_default_index_dir().join(dict.id.as_str());

        if !index_path.exists() {
            let index_args = IndexArgs {
                dictionary: args.dictionary.clone(),
                directory: None,
                overwrite: false,
                memory: DEFAULT_INDEX_MEMORY,
            };
            crate::index(ctx, &index_args).await?;
        }
    }

    let results = dict.search(args.query.as_str(), SearchOptions::default())?;

    print_entries(ctx, results, &args.format)?;

    Ok(())
}
