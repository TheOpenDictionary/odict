use clap::{arg, command, Args};
use odict::{
    download::DictionaryDownloader, index::get_default_index_dir, search::SearchOptions,
    LoadOptions, OpenDictionary,
};

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

    #[arg(
        short = 'r',
        long,
        default_value_t = crate::DEFAULT_RETRIES,
        help = "Number of times to retry loading the dictionary (remote-only)"
    )]
    retries: u32,
}

pub async fn search<'a>(ctx: &mut CLIContext<'a>, args: &SearchArgs) -> anyhow::Result<()> {
    let file = OpenDictionary::load_with_options(
        &args.dictionary,
        LoadOptions::default()
            .with_downloader(DictionaryDownloader::default().with_retries(args.retries)),
    )
    .await?;

    let dict = file.contents()?;

    if args.index {
        let index_path = get_default_index_dir().join(dict.id.as_str());

        if !index_path.exists() {
            let index_args = IndexArgs {
                dictionary: args.dictionary.clone(),
                directory: None,
                retries: crate::DEFAULT_RETRIES,
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
