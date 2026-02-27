use std::time::Duration;

use crate::enums::PrintFormat;
use crate::get_lookup_entries;
use crate::{context::CLIContext, print_entries};
use clap::{arg, command, Args};
use odict::{
    download::DictionaryDownloader,
    split::SplitOptions,
    LoadOptions, OpenDictionary,
};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct SplitArgs {
    #[arg(required = true, help = "Path to a compiled dictionary")]
    dictionary_path: String,

    #[arg(required = true, help = "Text to split into dictionary words")]
    queries: Vec<String>,

    #[arg(
        short,
        long,
        value_enum,
        default_value_t = PrintFormat::Print,
        help = "Output format of the entries"
    )]
    format: PrintFormat,

    #[arg(
        short = 'F',
        long,
        help = "Follow see_also redirects until finding an entry with etymologies"
    )]
    follow: bool,

    #[arg(
        short = 'm',
        long,
        default_value_t = 1,
        help = "Minimum character length of each split segment"
    )]
    min_length: usize,

    #[arg(
        short = 'i',
        long,
        default_value_t = false,
        help = "Perform case-insensitive lookups"
    )]
    insensitive: bool,

    #[arg(
        short = 'r',
        long,
        default_value_t = crate::DEFAULT_RETRIES,
        help = "Number of times to retry loading the dictionary (remote-only)"
    )]
    retries: u32,
}

pub async fn split<'a>(ctx: &mut CLIContext<'a>, args: &SplitArgs) -> anyhow::Result<()> {
    let SplitArgs {
        dictionary_path: path,
        queries,
        format,
        follow,
        min_length,
        insensitive,
        retries,
    } = args;

    let spinner = indicatif::ProgressBar::new_spinner();

    spinner.enable_steady_tick(Duration::from_millis(100));

    let file = OpenDictionary::load_with_options(
        path,
        LoadOptions::default()
            .with_downloader(DictionaryDownloader::default().with_retries(*retries)),
    )
    .await?;

    let opts = SplitOptions::default()
        .threshold(*min_length)
        .follow(*follow)
        .insensitive(*insensitive);

    let result = file.contents()?.split(queries, opts);

    spinner.finish_and_clear();

    match result {
        Ok(entries) => {
            print_entries(ctx, get_lookup_entries(entries), format)?;
            Ok(())
        }
        Err(err) => Err(anyhow::Error::from(err)),
    }
}
